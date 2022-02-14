use std::collections::{HashMap, HashSet};
use std::string::String;

use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use syn::*;

use crate::ir::IrType::*;
use crate::ir::*;

use crate::source_graph::{Enum, Struct};

use crate::parser::{extract_comments, type_to_string};

lazy_static! {
    static ref CAPTURE_RESULT: GenericCapture = GenericCapture::new("Result");
    static ref CAPTURE_OPTION: GenericCapture = GenericCapture::new("Option");
    static ref CAPTURE_BOX: GenericCapture = GenericCapture::new("Box");
    static ref CAPTURE_VEC: GenericCapture = GenericCapture::new("Vec");
    static ref CAPTURE_STREAM_SINK: GenericCapture = GenericCapture::new("StreamSink");
    static ref CAPTURE_ZERO_COPY_BUFFER: GenericCapture = GenericCapture::new("ZeroCopyBuffer");
    static ref CAPTURE_SYNC_RETURN: GenericCapture = GenericCapture::new("SyncReturn");
}

pub struct TypeParser<'a> {
    src_structs: HashMap<String, &'a Struct>,
    src_enums: HashMap<String, &'a Enum>,

    parsing_or_parsed_struct_names: HashSet<String>,
    struct_pool: IrStructPool,

    parsed_enums: HashSet<String>,
    enum_pool: IrEnumPool,
}

impl<'a> TypeParser<'a> {
    pub fn new(
        src_structs: HashMap<String, &'a Struct>,
        src_enums: HashMap<String, &'a Enum>,
    ) -> Self {
        TypeParser {
            src_structs,
            src_enums,
            struct_pool: HashMap::new(),
            enum_pool: HashMap::new(),
            parsing_or_parsed_struct_names: HashSet::new(),
            parsed_enums: HashSet::new(),
        }
    }

    pub fn consume(self) -> (IrStructPool, IrEnumPool) {
        (self.struct_pool, self.enum_pool)
    }
}

impl<'a> TypeParser<'a> {
    pub fn parse_type(&mut self, ty: &str) -> IrType {
        debug!("parse_type: {}", ty);
        None.or_else(|| self.try_parse_primitive(ty))
            .or_else(|| self.try_parse_api_type_delegate(ty))
            .or_else(|| self.try_parse_list(ty))
            .or_else(|| self.try_parse_box(ty))
            .or_else(|| self.try_parse_option(ty))
            .or_else(|| self.try_parse_struct(ty))
            .or_else(|| self.try_parse_enum(ty))
            .unwrap_or_else(|| panic!("parse_type failed for ty={}", ty))
    }

    pub fn try_parse_stream_sink(&mut self, ty: &str) -> Option<IrType> {
        CAPTURE_STREAM_SINK
            .captures(ty)
            .map(|inner| self.parse_type(&inner))
    }

    fn try_parse_primitive(&mut self, ty: &str) -> Option<IrType> {
        IrTypePrimitive::try_from_rust_str(ty).map(Primitive)
    }

    fn try_parse_api_type_delegate(&mut self, ty: &str) -> Option<IrType> {
        match ty {
            "SyncReturn<Vec<u8>>" => Some(IrType::Delegate(IrTypeDelegate::SyncReturnVecU8)),
            "String" => Some(IrType::Delegate(IrTypeDelegate::String)),
            "Vec<String>" => Some(IrType::Delegate(IrTypeDelegate::StringList)),
            _ => {
                if let Some(inner_type_str) = CAPTURE_ZERO_COPY_BUFFER.captures(ty) {
                    if let Some(IrType::PrimitiveList(IrTypePrimitiveList { primitive })) =
                        self.try_parse_list(&inner_type_str)
                    {
                        return Some(IrType::Delegate(
                            IrTypeDelegate::ZeroCopyBufferVecPrimitive(primitive),
                        ));
                    }
                }

                None
            }
        }
    }

    fn try_parse_list(&mut self, ty: &str) -> Option<IrType> {
        if let Some(inner_type_str) = CAPTURE_VEC.captures(ty) {
            match self.parse_type(&inner_type_str) {
                Primitive(primitive) => Some(PrimitiveList(IrTypePrimitiveList { primitive })),
                others => Some(GeneralList(IrTypeGeneralList {
                    inner: Box::new(others),
                })),
            }
        } else {
            None
        }
    }

    fn try_parse_box(&mut self, ty: &str) -> Option<IrType> {
        CAPTURE_BOX.captures(ty).map(|inner| {
            Boxed(IrTypeBoxed {
                exist_in_real_api: true,
                inner: Box::new(self.parse_type(&inner)),
            })
        })
    }

    fn try_parse_option(&mut self, ty: &str) -> Option<IrType> {
        CAPTURE_OPTION.captures(ty).map(|inner| {
            let inner_option = CAPTURE_OPTION.captures(&inner);
            if let Some(inner_option) = inner_option {
                panic!(
                    "Nested optionals without indirection are not supported. (Option<Option<{}>>)",
                    inner_option
                );
            };
            match self.parse_type(&inner) {
                Primitive(prim) => IrType::Optional(IrTypeOptional::new_prim(prim)),
                st @ StructRef(_) => {
                    IrType::Optional(IrTypeOptional::new_ptr(Boxed(IrTypeBoxed {
                        inner: Box::new(st),
                        exist_in_real_api: false,
                    })))
                }
                other => IrType::Optional(IrTypeOptional::new_ptr(other)),
            }
        })
    }

    fn try_parse_struct(&mut self, ty: &str) -> Option<IrType> {
        if !self.src_structs.contains_key(ty) {
            return None;
        }

        if !self.parsing_or_parsed_struct_names.contains(ty) {
            self.parsing_or_parsed_struct_names.insert(ty.to_string());
            let api_struct = self.parse_struct_core(ty);
            self.struct_pool.insert(ty.to_string(), api_struct);
        }

        Some(StructRef(IrTypeStructRef {
            name: ty.to_string(),
        }))
    }

    fn try_parse_enum(&mut self, ty: &str) -> Option<IrType> {
        if !self.src_enums.contains_key(ty) {
            return None;
        }

        if self.parsed_enums.insert(ty.to_owned()) {
            let enu = self.parse_enum_core(ty);
            self.enum_pool.insert(ty.to_owned(), enu);
        }

        Some(EnumRef(IrTypeEnumRef {
            name: ty.to_owned(),
            is_struct: self
                .enum_pool
                .get(ty)
                .map(IrEnum::is_struct)
                .unwrap_or(true),
        }))
    }
}

impl<'a> TypeParser<'a> {
    fn parse_enum_core(&mut self, ty: &str) -> IrEnum {
        let src_enum = self.src_enums[ty];
        let name = src_enum.ident.to_string();
        let path = src_enum.path.clone();
        let comments = extract_comments(&src_enum.src.attrs);
        let variants = src_enum
            .src
            .variants
            .iter()
            .map(|variant| IrVariant {
                name: IrIdent::new(variant.ident.to_string()),
                comments: extract_comments(&variant.attrs),
                kind: match variant.fields.iter().next() {
                    None => IrVariantKind::Value,
                    Some(Field {
                        attrs,
                        ident: field_ident,
                        ..
                    }) => {
                        let variant_ident = variant.ident.to_string();
                        IrVariantKind::Struct(IrStruct {
                            name: variant_ident,
                            path: None,
                            is_fields_named: field_ident.is_some(),
                            comments: extract_comments(attrs),
                            fields: variant
                                .fields
                                .iter()
                                .enumerate()
                                .map(|(idx, field)| IrField {
                                    name: IrIdent::new(
                                        field
                                            .ident
                                            .as_ref()
                                            .map(ToString::to_string)
                                            .unwrap_or_else(|| format!("field{}", idx)),
                                    ),
                                    ty: self.parse_type(&type_to_string(&field.ty)),
                                    comments: extract_comments(&field.attrs),
                                })
                                .collect(),
                        })
                    }
                },
            })
            .collect();
        IrEnum::new(name, path, comments, variants)
    }

    fn parse_struct_core(&mut self, ty: &str) -> IrStruct {
        let src_struct = self.src_structs[ty];
        let mut fields = Vec::new();

        let (is_fields_named, struct_fields) = match &src_struct.src.fields {
            Fields::Named(FieldsNamed { named, .. }) => (true, named),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
            _ => panic!("unsupported type: {:?}", src_struct.src.fields),
        };

        for (idx, field) in struct_fields.iter().enumerate() {
            let field_name = field
                .ident
                .as_ref()
                .map_or(format!("field{}", idx), ToString::to_string);
            let field_type_str = type_to_string(&field.ty);
            let field_type = self.parse_type(&field_type_str);
            fields.push(IrField {
                name: IrIdent::new(field_name),
                ty: field_type,
                comments: extract_comments(&field.attrs),
            });
        }

        let name = src_struct.ident.to_string();
        let path = Some(src_struct.path.clone());
        let comments = extract_comments(&src_struct.src.attrs);
        IrStruct {
            name,
            path,
            fields,
            is_fields_named,
            comments,
        }
    }
}

struct GenericCapture {
    regex: Regex,
}

impl GenericCapture {
    pub fn new(cls_name: &str) -> Self {
        let regex =
            Regex::new(&*format!("^[^<]*{}<([a-zA-Z0-9_<>()\\[\\];]+)>$", cls_name)).unwrap();
        Self { regex }
    }

    /// e.g. List<Tom> => return Some(Tom)
    pub fn captures(&self, s: &str) -> Option<String> {
        self.regex
            .captures(s)
            .iter()
            .find_map(|capture| capture.get(1))
            .map(|inner| inner.as_str().to_owned())
    }
}
