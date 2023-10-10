use crate::ir::IrType::*;
use crate::ir::*;
use crate::parser::markers;
use crate::parser::source_graph::{Enum, Struct};
use crate::parser::{extract_comments, extract_metadata};
use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use std::string::String;
use syn::punctuated::Punctuated;
use syn::*;

use super::DefaultValues;

pub struct TypeParser<'a> {
    src_structs: HashMap<String, &'a Struct>,
    src_enums: HashMap<String, &'a Enum>,
    src_types: HashMap<String, Type>,
    parsing_or_parsed_struct_names: HashSet<String>,
    struct_pool: IrStructPool,
    parsed_enums: HashSet<String>,
    enum_pool: IrEnumPool,
}

impl<'a> TypeParser<'a> {
    pub fn new(
        src_structs: HashMap<String, &'a Struct>,
        src_enums: HashMap<String, &'a Enum>,
        src_types: HashMap<String, Type>,
    ) -> Self {
        TypeParser {
            src_structs,
            src_enums,
            src_types,
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

pub fn convert_ident_str(ty: &Type) -> Option<String> {
    if let Type::Path(TypePath { qself: _, path }) = ty {
        if let Some(PathSegment {
            ident,
            arguments: _,
        }) = path.segments.first()
        {
            return Some(ident.to_string());
        }
    }

    // Unhandled case, return None
    None
}

#[cfg(feature = "chrono")]
fn datetime_to_ir_type(args: &[IrType]) -> std::result::Result<IrType, String> {
    if let [Unencodable(IrTypeUnencodable { segments, .. })] = args {
        let mut segments = segments.clone();
        let segments: Vec<NameComponent> = if cfg!(feature = "qualified_names") {
            segments
        } else {
            // Emulate old behavior by discarding any name qualifiers
            vec![segments.pop().unwrap()]
        };

        let splayed = segments.splay();
        return match splayed[..] {
            #[cfg(feature = "qualified_names")]
            [("DateTime", None), ("Utc", None)] => {
                Ok(Delegate(IrTypeDelegate::Time(IrTypeTime::Utc)))
            }

            [("Utc", None)] => Ok(Delegate(IrTypeDelegate::Time(IrTypeTime::Utc))),

            #[cfg(feature = "qualified_names")]
            [("DateTime", None), ("Local", None)] => {
                Ok(Delegate(IrTypeDelegate::Time(IrTypeTime::Local)))
            }

            [("Local", None)] => Ok(Delegate(IrTypeDelegate::Time(IrTypeTime::Local))),

            _ => Err("Invalid DateTime generic".to_string()),
        };
    }
    Err("Invalid DateTime generic".to_string())
}

fn path_type_to_unencodable(
    type_path: &TypePath,
    flat_vector: Vec<(&str, Option<ArgsRefs>)>,
) -> IrType {
    Unencodable(IrTypeUnencodable {
        string: type_path.to_token_stream().to_string(),
        segments: flat_vector
            .iter()
            .map(|(ident, option_args_refs)| NameComponent {
                ident: ident.to_string(),
                args: option_args_refs.as_ref().map(|args_refs| match args_refs {
                    ArgsRefs::Generic(args_array) => Args::Generic(args_array.to_vec()),
                    ArgsRefs::Signature(args_array) => Args::Signature(args_array.to_vec()),
                }),
            })
            .collect(),
    })
}

impl<'a> TypeParser<'a> {
    pub fn resolve_alias<'b: 'a>(&self, ty: &'b Type) -> &Type {
        self.get_alias_type(ty).unwrap_or(ty)
    }

    pub fn get_alias_type(&self, ty: &syn::Type) -> Option<&Type> {
        convert_ident_str(ty).and_then(|key| self.src_types.get(&key))
    }

    pub fn parse_type(&mut self, ty: &syn::Type) -> IrType {
        let resolve_ty = self.resolve_alias(ty).clone();

        match resolve_ty.clone() {
            syn::Type::Path(path) => self.convert_path_to_ir_type(&path).unwrap(),
            syn::Type::Array(syn::TypeArray { elem, len, .. }) => {
                let length: usize = match len {
                    syn::Expr::Lit(lit) => match &lit.lit {
                        syn::Lit::Int(x) => x.base10_parse().unwrap(),
                        _ => panic!("Cannot parse array length"),
                    },
                    _ => panic!("Cannot parse array length"),
                };
                match self.parse_type(&elem) {
                    Primitive(primitive) => {
                        Delegate(IrTypeDelegate::Array(IrTypeDelegateArray::PrimitiveArray {
                            length,
                            primitive,
                        }))
                    }
                    others => Delegate(IrTypeDelegate::Array(IrTypeDelegateArray::GeneralArray {
                        length,
                        general: Box::new(others),
                    })),
                }
            }
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                if elems.is_empty() {
                    IrType::Primitive(IrTypePrimitive::Unit)
                } else {
                    self.convert_tuple_to_ir_type(elems)
                }
            }
            _ => IrType::Unencodable(IrTypeUnencodable {
                string: resolve_ty.to_token_stream().to_string(),
                segments: vec![],
            }),
        }
    }

    fn angle_bracketed_generic_arguments_to_ir_types(
        &mut self,
        args: &AngleBracketedGenericArguments,
    ) -> std::result::Result<Vec<IrType>, String> {
        let AngleBracketedGenericArguments { args, .. } = args;
        args.iter()
            .filter_map(|arg| {
                if let GenericArgument::Type(ty) = arg {
                    Some(Ok(self.parse_type(ty)))
                } else {
                    None
                }
            })
            .collect()
    }

    fn parenthesized_generic_arguments_to_ir_types(
        &mut self,
        args: &ParenthesizedGenericArguments,
    ) -> Vec<IrType> {
        let ParenthesizedGenericArguments { inputs, output, .. } = args;

        let mut args = inputs
            .iter()
            .map(|ty| self.parse_type(ty))
            .collect::<Vec<IrType>>();

        match output {
            syn::ReturnType::Default => {
                args.insert(0, Primitive(IrTypePrimitive::Unit));
            }
            syn::ReturnType::Type(_, ret_ty) => {
                args.insert(0, self.parse_type(ret_ty));
            }
        };

        args
    }

    fn path_data(&mut self, path: &Path) -> std::result::Result<Vec<NameComponent>, String> {
        let Path { segments, .. } = path;

        let data: std::result::Result<Vec<NameComponent>, String> = segments
            .iter()
            .map(|segment| {
                let ident = segment.ident.to_string();
                match &segment.arguments {
                    PathArguments::None => Ok(NameComponent { ident, args: None }),
                    PathArguments::AngleBracketed(args) => {
                        match self.angle_bracketed_generic_arguments_to_ir_types(args) {
                            Err(sub_err) => Err(format!(
                                "\"{}\" of \"{}\" is not valid. {}",
                                ident,
                                path.to_token_stream(),
                                sub_err
                            )),
                            Ok(ir_types) => Ok(NameComponent {
                                ident,
                                args: Some(Args::Generic(ir_types)),
                            }),
                        }
                    }
                    PathArguments::Parenthesized(args) => Ok(NameComponent {
                        ident,
                        args: Some(Args::Signature(
                            self.parenthesized_generic_arguments_to_ir_types(args),
                        )),
                    }),
                }
            })
            .collect();

        let storage = data?;

        Ok(storage)
    }

    /// Converts a path type into an `IrType` if possible.
    pub fn convert_path_to_ir_type(
        &mut self,
        type_path: &TypePath,
    ) -> std::result::Result<IrType, String> {
        match &type_path {
            TypePath { qself: None, path } => {
                let segments: Vec<NameComponent> = if cfg!(feature = "qualified_names") {
                    self.path_data(path)?
                } else {
                    // Emulate old behavior by discarding any name qualifiers
                    vec![self.path_data(path)?.pop().unwrap()]
                };

                use ArgsRefs::*;

                let flat_vector = segments.splay();
                let flat_array = &flat_vector[..];
                match flat_array {
                    // Non generic types
                    #[cfg(all(feature = "chrono", feature = "qualified_names"))]
                    [("chrono", None), ("Duration", None)] => {
                        Ok(Delegate(IrTypeDelegate::Time(IrTypeTime::Duration)))
                    }

                    #[cfg(all(feature = "chrono", not(feature = "qualified_names")))]
                    [("Duration", None)] => {
                        Ok(Delegate(IrTypeDelegate::Time(IrTypeTime::Duration)))
                    }

                    #[cfg(all(feature = "chrono", feature = "qualified_names"))]
                    [("chrono", None), ("NaiveDateTime", None)] => {
                        Ok(Delegate(IrTypeDelegate::Time(IrTypeTime::Naive)))
                    }

                    #[cfg(all(feature = "chrono", not(feature = "qualified_names")))]
                    [("NaiveDateTime", None)] => {
                        Ok(Delegate(IrTypeDelegate::Time(IrTypeTime::Naive)))
                    }

                    #[cfg(feature = "qualified_names")]
                    [("flutter_rust_bridge", None), ("DartAbi", None)] => {
                        Ok(Dynamic(IrTypeDynamic))
                    }

                    [("DartAbi", None)] => Ok(Dynamic(IrTypeDynamic)),

                    #[cfg(all(feature = "uuid", feature = "qualified_names"))]
                    [("uuid", None), ("Uuid", None)] => Ok(Delegate(IrTypeDelegate::Uuid)),

                    #[cfg(all(feature = "uuid", not(feature = "qualified_names")))]
                    [("Uuid", None)] => Ok(Delegate(IrTypeDelegate::Uuid)),

                    #[cfg(feature = "qualified_names")]
                    [("flutter_rust_bridge", None), ("DartOpaque", None)] => {
                        Ok(DartOpaque(IrTypeDartOpaque {}))
                    }

                    [("DartOpaque", None)] => Ok(DartOpaque(IrTypeDartOpaque {})),

                    [("String", None)] => Ok(Delegate(IrTypeDelegate::String)),

                    [("Backtrace", None)] => Ok(Delegate(IrTypeDelegate::Backtrace)),

                    // TODO: change to "if let guard" https://github.com/rust-lang/rust/issues/51114
                    [(name, None)]
                        if matches!(IrTypePrimitive::try_from_rust_str(name), Some(..)) =>
                    {
                        Ok(Primitive(IrTypePrimitive::try_from_rust_str(name).unwrap()))
                    }

                    [(name, None)] if self.src_structs.contains_key(&name.to_string()) => {
                        let ident_string = name.to_string();
                        if !self.parsing_or_parsed_struct_names.contains(&ident_string) {
                            self.parsing_or_parsed_struct_names
                                .insert(ident_string.to_owned());
                            let api_struct = match self.parse_struct_core(&ident_string) {
                                Some(ir_struct) => ir_struct,
                                None => {
                                    return Ok(path_type_to_unencodable(type_path, flat_vector))
                                }
                            };
                            self.struct_pool.insert(ident_string.to_owned(), api_struct);
                        }

                        Ok(StructRef(IrTypeStructRef {
                            name: ident_string.to_owned(),
                            freezed: self
                                .struct_pool
                                .get(&ident_string)
                                .map(IrStruct::using_freezed)
                                .unwrap_or(false),
                            empty: self
                                .struct_pool
                                .get(&ident_string)
                                .map(IrStruct::is_empty)
                                .unwrap_or(false),
                            is_exception: false,
                        }))
                    }

                    [(name, _)] if self.src_enums.contains_key(&name.to_string()) => {
                        let ident_string = name.to_string();
                        if self.parsed_enums.insert(ident_string.to_owned()) {
                            let enu = self.parse_enum_core(&ident_string);
                            self.enum_pool.insert(ident_string.to_owned(), enu);
                        }

                        let enum_ref = IrTypeEnumRef {
                            name: ident_string.to_owned(),
                            is_exception: false,
                        };
                        let enu = self.enum_pool.get(&ident_string);
                        let is_struct = enu.map(IrEnum::is_struct).unwrap_or(true);
                        if is_struct {
                            Ok(EnumRef(enum_ref))
                        } else {
                            Ok(Delegate(IrTypeDelegate::PrimitiveEnum {
                                ir: enum_ref,
                                // TODO(Desdaemon): Parse #[repr] from enum
                                repr: IrTypePrimitive::I32,
                            }))
                        }
                    }

                    // Generic types
                    #[cfg(feature = "qualified_names")]
                    [("flutter_rust_bridge", None), ("SyncReturn", Some(Generic([element])))] => {
                        Ok(SyncReturn(IrTypeSyncReturn::new(element.clone())))
                    }

                    [("SyncReturn", Some(Generic([element])))] => {
                        Ok(SyncReturn(IrTypeSyncReturn::new(element.clone())))
                    }

                    [("Vec", Some(Generic([Delegate(IrTypeDelegate::String)])))] => {
                        Ok(Delegate(IrTypeDelegate::StringList))
                    }

                    #[cfg(feature = "uuid")]
                    [("Vec", Some(Generic([Delegate(IrTypeDelegate::Uuid)])))] => {
                        Ok(Delegate(IrTypeDelegate::Uuids))
                    }

                    [("Vec", Some(Generic([Optional(opt)])))] => {
                        if matches!(opt.inner.as_ref(), IrType::Optional(_)) {
                            Err(format!(
                                "Nested optionals without indirection are not allowed. {}",
                                type_path.to_token_stream()
                            ))?
                        }
                        Ok(OptionalList(IrTypeOptionalList {
                            inner: opt.inner.clone(),
                        }))
                    }

                    [("Vec", Some(Generic([Primitive(primitive)])))] => {
                        // Since Dart doesn't have a boolean primitive list like `Uint8List`,
                        // we need to convert `Vec<bool>` to a boolean general list in order to achieve the binding.
                        if primitive == &IrTypePrimitive::Bool {
                            Ok(GeneralList(IrTypeGeneralList {
                                inner: Box::new(IrType::Primitive(IrTypePrimitive::Bool)),
                            }))
                        } else {
                            Ok(PrimitiveList(IrTypePrimitiveList {
                                primitive: primitive.clone(),
                            }))
                        }
                    }

                    #[cfg(feature = "chrono")]
                    [("Vec", Some(Generic([Delegate(IrTypeDelegate::Time(time))])))] => {
                        Ok(Delegate(IrTypeDelegate::TimeList(*time)))
                    }

                    [("Vec", Some(Generic([element])))] => Ok(GeneralList(IrTypeGeneralList {
                        inner: Box::new(element.clone()),
                    })),

                    #[cfg(feature = "qualified_names")]
                    [("flutter_rust_bridge", None), (
                        "ZeroCopyBuffer",
                        Some(Generic([PrimitiveList(IrTypePrimitiveList { primitive })])),
                    )] => Ok(Delegate(IrTypeDelegate::ZeroCopyBufferVecPrimitive(
                        primitive.clone(),
                    ))),

                    [(
                        "ZeroCopyBuffer",
                        Some(Generic([PrimitiveList(IrTypePrimitiveList { primitive })])),
                    )] => Ok(Delegate(IrTypeDelegate::ZeroCopyBufferVecPrimitive(
                        primitive.clone(),
                    ))),

                    #[cfg(feature = "qualified_names")]
                    [("flutter_rust_bridge", None), (
                        "RustOpaque",
                        Some(Generic([Delegate(IrTypeDelegate::Array(array_delegate))])),
                    )] => Ok(Delegate(IrTypeDelegate::Array(array_delegate.clone()))),

                    [(
                        "RustOpaque",
                        Some(Generic([Delegate(IrTypeDelegate::Array(array_delegate))])),
                    )] => Ok(Delegate(IrTypeDelegate::Array(array_delegate.clone()))),

                    #[cfg(feature = "qualified_names")]
                    [("flutter_rust_bridge", None), ("RustOpaque", Some(Generic([Primitive(IrTypePrimitive::Unit)])))] => {
                        Ok(RustOpaque(IrTypeRustOpaque::new_unit()))
                    }

                    [("RustOpaque", Some(Generic([Primitive(IrTypePrimitive::Unit)])))] => {
                        Ok(RustOpaque(IrTypeRustOpaque::new_unit()))
                    }

                    #[cfg(feature = "qualified_names")]
                    [("flutter_rust_bridge", None), ("RustOpaque", Some(Generic([ty])))] => {
                        Ok(RustOpaque(IrTypeRustOpaque::from(ty.rust_api_type())))
                    }

                    [("RustOpaque", Some(Generic([ty])))] => {
                        Ok(RustOpaque(IrTypeRustOpaque::from(ty.rust_api_type())))
                    }

                    [("Box", Some(Generic([inner])))] => Ok(Boxed(IrTypeBoxed {
                        exist_in_real_api: true,
                        inner: Box::new(inner.clone()),
                    })),

                    [("Option", Some(Generic([Optional(_)])))] => Err(format!(
                        "Nested optionals without indirection are not supported. {}",
                        type_path.to_token_stream()
                    )),

                    [("Option", Some(Generic([SyncReturn(_)])))] => Err(format!(
                        "Option<SyncReturn<_>> has no effect. Consider SyncReturn<Option<_>> instead. {}",
                        type_path.to_token_stream()
                    )),

                    [("Option", Some(Generic([inner])))] => Ok(Optional(match inner {
                        StructRef(..)
                        | EnumRef(..)
                        | RustOpaque(..)
                        | DartOpaque(..)
                        | Primitive(..)
                        | Record(..)
                        | Delegate(IrTypeDelegate::PrimitiveEnum { .. }) => {
                            IrTypeOptional::new_boxed(inner.clone())
                        }
                        #[cfg(feature = "chrono")]
                        Delegate(IrTypeDelegate::Time(..)) => {
                            IrTypeOptional::new_boxed(inner.clone())
                        }
                        OptionalList(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_) | Dynamic(_) | Unencodable(_) | Delegate(_) => {
                            IrTypeOptional::new(inner.clone())
                        }
                        Optional(_) | SyncReturn(_) => unreachable!(),
                    })),

                    #[cfg(all(feature = "chrono", feature = "qualified_names"))]
                    [("chrono", None), ("DateTime", Some(Generic(args)))] => {
                        datetime_to_ir_type(args)
                    }

                    #[cfg(feature = "chrono")]
                    [("DateTime", Some(Generic(args)))] => datetime_to_ir_type(args),

                    _ => Ok(path_type_to_unencodable(type_path, flat_vector)),
                }
            }
            TypePath {
                qself: Some(QSelf { ty, .. }),
                ..
            } => Err(format!(
                "qself \"<{}>\" in \"{}\", and all qself syntax, is unsupported",
                ty.to_token_stream(),
                type_path.to_token_stream()
            )),
        }
    }

    fn convert_tuple_to_ir_type(&mut self, elems: Punctuated<Type, Token![,]>) -> IrType {
        let values = elems
            .iter()
            .map(|elem| self.parse_type(elem))
            .collect::<Vec<_>>();
        let safe_ident = values
            .iter()
            .map(IrType::safe_ident)
            .collect::<Vec<_>>()
            .join("_");
        let safe_ident = format!("__record__{safe_ident}");
        self.struct_pool.insert(
            safe_ident.clone(),
            IrStruct {
                name: safe_ident.clone(),
                wrapper_name: None,
                path: None,
                is_fields_named: true,
                dart_metadata: vec![],
                comments: vec![],
                fields: values
                    .iter()
                    .enumerate()
                    .map(|(idx, ty)| IrField {
                        ty: ty.clone(),
                        name: IrIdent::new(format!("field{idx}")),
                        is_final: true,
                        comments: vec![],
                        default: None,
                        settings: Default::default(),
                    })
                    .collect(),
            },
        );
        IrType::Record(IrTypeRecord {
            inner: IrTypeStructRef {
                name: safe_ident,
                freezed: false,
                empty: false,
                is_exception: false,
            },
            values: values.into_boxed_slice(),
        })
    }

    fn parse_enum_core(&mut self, ident_string: &String) -> IrEnum {
        let src_enum = self.src_enums[ident_string];
        let name = src_enum.ident.to_string();
        let wrapper_name = if src_enum.mirror {
            Some(format!("mirror_{name}"))
        } else {
            None
        };

        let path = src_enum.path.clone();
        let comments = extract_comments(&src_enum.src.attrs);
        let variants = src_enum
            .src
            .variants
            .iter()
            .map(|variant| IrVariant {
                name: IrIdent::new(variant.ident.to_string()),
                wrapper_name: IrIdent::new(format!("{}_{}", src_enum.ident, variant.ident)),
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
                            wrapper_name: None,
                            path: None,
                            is_fields_named: field_ident.is_some(),
                            dart_metadata: extract_metadata(attrs),
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
                                            .unwrap_or_else(|| format!("field{idx}")),
                                    ),
                                    ty: self.parse_type(&field.ty),
                                    is_final: true,
                                    comments: extract_comments(&field.attrs),
                                    default: DefaultValues::extract(&field.attrs),
                                    settings: IrFieldSettings {
                                        is_in_mirrored_enum: src_enum.mirror,
                                    },
                                })
                                .collect(),
                        })
                    }
                },
            })
            .collect();
        IrEnum::new(name, wrapper_name, path, comments, variants, false)
    }

    fn parse_struct_core(&mut self, ident_string: &String) -> Option<IrStruct> {
        let src_struct = self.src_structs[ident_string];
        let mut fields = Vec::new();
        let (is_fields_named, struct_fields) = match &src_struct.src.fields {
            Fields::Named(FieldsNamed { named, .. }) => (true, named),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (false, unnamed),
            _ => return None,
        };
        for (idx, field) in struct_fields.iter().enumerate() {
            let field_name = field
                .ident
                .as_ref()
                .map_or(format!("field{idx}"), ToString::to_string);
            let field_type = self.parse_type(&field.ty);
            fields.push(IrField {
                name: IrIdent::new(field_name),
                ty: field_type,
                is_final: !markers::has_non_final(&field.attrs),
                comments: extract_comments(&field.attrs),
                default: DefaultValues::extract(&field.attrs),
                settings: IrFieldSettings::default(),
            });
        }
        let name = src_struct.ident.to_string();
        let wrapper_name = if src_struct.mirror {
            Some(format!("mirror_{name}"))
        } else {
            None
        };
        let path = Some(src_struct.path.clone());
        let metadata = extract_metadata(&src_struct.src.attrs);
        let comments = extract_comments(&src_struct.src.attrs);
        Some(IrStruct {
            name,
            wrapper_name,
            path,
            fields,
            is_fields_named,
            dart_metadata: metadata,
            comments,
        })
    }
}
