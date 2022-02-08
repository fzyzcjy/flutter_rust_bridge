use std::collections::{HashMap, HashSet};
use std::string::String;

use lazy_static::lazy_static;
use log::debug;
use quote::quote;
use regex::Regex;
use syn::*;

use ApiType::*;

use crate::api_types::*;
use crate::generator_rust::HANDLER_NAME;
use crate::source_graph::{Crate, Enum, Struct};

lazy_static! {
    static ref CAPTURE_RESULT: GenericCapture = GenericCapture::new("Result");
    static ref CAPTURE_OPTION: GenericCapture = GenericCapture::new("Option");
    static ref CAPTURE_BOX: GenericCapture = GenericCapture::new("Box");
    static ref CAPTURE_VEC: GenericCapture = GenericCapture::new("Vec");
    static ref CAPTURE_STREAM_SINK: GenericCapture = GenericCapture::new("StreamSink");
    static ref CAPTURE_ZERO_COPY_BUFFER: GenericCapture = GenericCapture::new("ZeroCopyBuffer");
    static ref CAPTURE_SYNC_RETURN: GenericCapture = GenericCapture::new("SyncReturn");
    static ref ALL_CAPTURES: Vec<&'static GenericCapture> = vec![
        &CAPTURE_RESULT,
        &CAPTURE_OPTION,
        &CAPTURE_BOX,
        &CAPTURE_VEC,
        &CAPTURE_STREAM_SINK,
        &CAPTURE_ZERO_COPY_BUFFER,
        &CAPTURE_SYNC_RETURN
    ];
}

pub fn parse(source_rust_content: &str, file: File, manifest_path: &String) -> ApiFile {
    let crate_map = Crate::new(manifest_path);

    let src_fns = extract_fns_from_file(&file);

    let mut src_structs = HashMap::new();
    crate_map.root_module.collect_structs(&mut src_structs);

    let mut src_enums = HashMap::new();
    crate_map.root_module.collect_enums(&mut src_enums);

    let parser = Parser {
        src_structs,
        src_enums,
        struct_pool: HashMap::new(),
        enum_pool: HashMap::new(),
        parsing_or_parsed_struct_names: HashSet::new(),
        parsed_enums: HashSet::new(),
    };
    parser.parse(source_rust_content, src_fns)
}

struct Parser<'a> {
    src_structs: HashMap<String, &'a Struct>,
    parsing_or_parsed_struct_names: HashSet<String>,
    struct_pool: ApiStructPool,

    src_enums: HashMap<String, &'a Enum>,
    parsed_enums: HashSet<String>,
    enum_pool: ApiEnumPool,
}

fn extract_comments(attrs: &[Attribute]) -> Vec<Comment> {
    attrs
        .iter()
        .filter_map(|attr| match attr.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit),
                ..
            })) if path.is_ident("doc") => Some(Comment::from(lit.value().as_ref())),
            _ => None,
        })
        .collect()
}

impl<'a> Parser<'a> {
    fn parse(mut self, source_rust_content: &str, src_fns: Vec<&ItemFn>) -> ApiFile {
        let funcs = src_fns.iter().map(|f| self.parse_function(f)).collect();

        let has_executor = source_rust_content.contains(HANDLER_NAME);

        ApiFile {
            funcs,
            struct_pool: self.struct_pool,
            enum_pool: self.enum_pool,
            has_executor,
        }
    }

    fn parse_function(&mut self, func: &ItemFn) -> ApiFunc {
        debug!("parse_function function name: {:?}", func.sig.ident);

        let sig = &func.sig;
        let func_name = sig.ident.to_string();

        let mut inputs = Vec::new();
        let mut output = None;
        let mut mode = None;

        for sig_input in &sig.inputs {
            if let FnArg::Typed(ref pat_type) = sig_input {
                let name = if let Pat::Ident(ref pat_ident) = *pat_type.pat {
                    format!("{}", pat_ident.ident)
                } else {
                    panic!("unexpected pat_type={:?}", pat_type)
                };
                let type_string = type_to_string(&pat_type.ty);

                if let Some(stream_sink_inner_type) = self.try_parse_stream_sink(&type_string) {
                    output = Some(stream_sink_inner_type);
                    mode = Some(ApiFuncMode::Stream);
                } else {
                    inputs.push(ApiField {
                        name: ApiIdent::new(name),
                        ty: self.parse_type(&type_string),
                        comments: extract_comments(&pat_type.attrs),
                    });
                }
            } else {
                panic!("unexpected sig_input={:?}", sig_input);
            }
        }

        if output.is_none() {
            output = Some(if let ReturnType::Type(_, ty) = &sig.output {
                let type_string = type_to_string(ty);
                if let Some(inner) = CAPTURE_RESULT.captures(&type_string) {
                    self.parse_type(&inner)
                } else {
                    panic!("unsupported type_string: {}", type_string);
                }
            } else {
                panic!("unsupported output: {:?}", sig.output);
            });
            mode = Some(
                if let Some(ApiType::Delegate(ApiTypeDelegate::SyncReturnVecU8)) = output {
                    ApiFuncMode::Sync
                } else {
                    ApiFuncMode::Normal
                },
            );
        }

        // let comments = func.attrs.iter().filter_map(extract_comments).collect();

        ApiFunc {
            name: func_name,
            inputs,
            output: output.expect("unsupported output"),
            mode: mode.expect("unsupported mode"),
            comments: extract_comments(&func.attrs),
        }
    }

    fn parse_type(&mut self, ty: &str) -> ApiType {
        debug!("parse_type: {}", ty);
        None.or_else(|| ApiTypePrimitive::try_from_rust_str(ty).map(Primitive))
            .or_else(|| self.try_parse_api_type_delegate(ty))
            .or_else(|| self.try_parse_list(ty))
            .or_else(|| self.try_parse_box(ty))
            .or_else(|| self.try_parse_option(ty))
            .or_else(|| self.try_parse_struct(ty))
            .or_else(|| self.try_parse_enum(ty))
            .unwrap_or_else(|| panic!("parse_type failed for ty={}", ty))
    }

    fn try_parse_stream_sink(&mut self, ty: &str) -> Option<ApiType> {
        CAPTURE_STREAM_SINK
            .captures(ty)
            .map(|inner| self.parse_type(&inner))
    }

    fn try_parse_api_type_delegate(&mut self, ty: &str) -> Option<ApiType> {
        match ty {
            "SyncReturn<Vec<u8>>" => Some(ApiType::Delegate(ApiTypeDelegate::SyncReturnVecU8)),
            "String" => Some(ApiType::Delegate(ApiTypeDelegate::String)),
            "Vec<String>" => Some(ApiType::Delegate(ApiTypeDelegate::StringList)),
            _ => {
                if let Some(inner_type_str) = CAPTURE_ZERO_COPY_BUFFER.captures(ty) {
                    if let Some(ApiType::PrimitiveList(ApiTypePrimitiveList { primitive })) =
                        self.try_parse_list(&inner_type_str)
                    {
                        return Some(ApiType::Delegate(
                            ApiTypeDelegate::ZeroCopyBufferVecPrimitive(primitive),
                        ));
                    }
                }

                None
            }
        }
    }

    fn try_parse_list(&mut self, ty: &str) -> Option<ApiType> {
        if let Some(inner_type_str) = CAPTURE_VEC.captures(ty) {
            match self.parse_type(&inner_type_str) {
                Primitive(primitive) => Some(PrimitiveList(ApiTypePrimitiveList { primitive })),
                others => Some(GeneralList(Box::from(ApiTypeGeneralList { inner: others }))),
            }
        } else {
            None
        }
    }

    fn try_parse_box(&mut self, ty: &str) -> Option<ApiType> {
        CAPTURE_BOX.captures(ty).map(|inner| {
            Boxed(Box::new(ApiTypeBoxed {
                exist_in_real_api: true,
                inner: self.parse_type(&inner),
            }))
        })
    }

    fn try_parse_option(&mut self, ty: &str) -> Option<ApiType> {
        CAPTURE_OPTION.captures(ty).map(|inner| {
            let inner_option = CAPTURE_OPTION.captures(&inner);
            if let Some(inner_option) = inner_option {
                panic!(
                    "Nested optionals without indirection are not supported. (Option<Option<{}>>)",
                    inner_option
                );
            };
            match self.parse_type(&inner) {
                Primitive(prim) => ApiType::Optional(ApiTypeOptional::new_prim(prim)),
                st @ StructRef(_) => {
                    ApiType::Optional(ApiTypeOptional::new_ptr(Boxed(Box::new(ApiTypeBoxed {
                        inner: st,
                        exist_in_real_api: false,
                    }))))
                }
                other => ApiType::Optional(ApiTypeOptional::new_ptr(other)),
            }
        })
    }

    fn try_parse_struct(&mut self, ty: &str) -> Option<ApiType> {
        if !self.src_structs.contains_key(ty) {
            return None;
        }

        if !self.parsing_or_parsed_struct_names.contains(ty) {
            self.parsing_or_parsed_struct_names.insert(ty.to_string());
            let api_struct = self.parse_struct_core(ty);
            self.struct_pool.insert(ty.to_string(), api_struct);
        }

        Some(StructRef(ApiTypeStructRef {
            name: ty.to_string(),
        }))
    }

    fn try_parse_enum(&mut self, ty: &str) -> Option<ApiType> {
        if !self.src_enums.contains_key(ty) {
            return None;
        }

        if self.parsed_enums.insert(ty.to_owned()) {
            let enu = self.parse_enum(ty);
            self.enum_pool.insert(ty.to_owned(), enu);
        }

        Some(EnumRef(ApiTypeEnumRef {
            name: ty.to_owned(),
            is_struct: self
                .enum_pool
                .get(ty)
                .map(ApiEnum::is_struct)
                .unwrap_or(true),
        }))
    }

    fn parse_enum(&mut self, ty: &str) -> ApiEnum {
        let src_enum = self.src_enums[ty];
        let name = src_enum.ident.to_string();
        let path = src_enum.path.clone();
        let comments = extract_comments(&src_enum.src.attrs);
        let variants = src_enum
            .src
            .variants
            .iter()
            .map(|variant| ApiVariant {
                name: ApiIdent::new(variant.ident.to_string()),
                comments: extract_comments(&variant.attrs),
                kind: match variant.fields.iter().next() {
                    None => ApiVariantKind::Value,
                    Some(Field {
                        attrs,
                        ident: field_ident,
                        ..
                    }) => {
                        let variant_ident = variant.ident.to_string();
                        ApiVariantKind::Struct(ApiStruct {
                            name: variant_ident,
                            path: None,
                            is_fields_named: field_ident.is_some(),
                            comments: extract_comments(attrs),
                            fields: variant
                                .fields
                                .iter()
                                .enumerate()
                                .map(|(idx, field)| ApiField {
                                    name: ApiIdent::new(
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
        ApiEnum::new(name, path, comments, variants)
    }

    fn parse_struct_core(&mut self, ty: &str) -> ApiStruct {
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
            fields.push(ApiField {
                name: ApiIdent::new(field_name),
                ty: field_type,
                comments: extract_comments(&field.attrs),
            });
        }

        let name = src_struct.ident.to_string();
        let path = Some(src_struct.path.clone());
        let comments = extract_comments(&src_struct.src.attrs);
        ApiStruct {
            name,
            path,
            fields,
            is_fields_named,
            comments,
        }
    }
}

fn extract_fns_from_file(file: &File) -> Vec<&ItemFn> {
    let mut src_fns = Vec::new();

    for item in file.items.iter() {
        match item {
            Item::Fn(ref item_fn) => {
                if let Visibility::Public(_) = &item_fn.vis {
                    src_fns.push(item_fn);
                }
            }
            _ => {}
        }
    }

    src_fns
}

/// syn -> string https://github.com/dtolnay/syn/issues/294
fn type_to_string(ty: &Type) -> String {
    quote!(#ty).to_string().replace(' ', "")
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
