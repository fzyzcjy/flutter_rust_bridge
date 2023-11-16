use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumIdent, IrTypeEnumRef};
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::optional::IrTypeOptional;
use crate::codegen::ir::ty::optional_list::IrTypeOptionalList;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::structure::{IrStruct, IrStructIdent, IrTypeStructRef};
use crate::codegen::ir::ty::unencodable::{Args, IrTypeUnencodable, NameComponent};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{
    Boxed, DartOpaque, Delegate, Dynamic, EnumRef, GeneralList, Optional, OptionalList, Primitive,
    PrimitiveList, Record, RustOpaque, StructRef, Unencodable,
};
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::parser::unencodable::{ArgsRefs, Splayable};
use anyhow::{anyhow, bail};
use quote::ToTokens;
use syn::{Path, QSelf, TypePath};

impl<'a> TypeParser<'a> {
    pub(crate) fn parse_type_path(&mut self, type_path: &TypePath) -> anyhow::Result<IrType> {
        match &type_path {
            TypePath { qself: None, path } => self.parse_type_path_core(type_path, path),
            TypePath {
                qself: Some(QSelf { ty, .. }),
                ..
            } => Err(anyhow!(
                "qself \"<{}>\" in \"{}\", and all qself syntax, is unsupported",
                ty.to_token_stream(),
                type_path.to_token_stream()
            )),
        }
    }

    fn parse_type_path_core(
        &mut self,
        type_path: &TypePath,
        path: &Path,
    ) -> anyhow::Result<IrType> {
        let segments: Vec<NameComponent> = if cfg!(feature = "qualified_names") {
            self.extract_path_data(path)?
        } else {
            // Emulate old behavior by discarding any name qualifiers
            vec![self.extract_path_data(path)?.pop().unwrap()]
        };

        use ArgsRefs::*;

        let flat_vector = segments.splay();
        let flat_array = &flat_vector[..];
        match flat_array {
            // Non generic types
            #[cfg(all(feature = "qualified_names"))]
            [("chrono", None), ("Duration", None)] => {
                Ok(Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Duration)))
            }

            #[cfg(all(not(feature = "qualified_names")))]
            [("Duration", None)] => {
                Ok(Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Duration)))
            }

            #[cfg(all(feature = "qualified_names"))]
            [("chrono", None), ("NaiveDateTime", None)] => {
                Ok(Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Naive)))
            }

            #[cfg(all(not(feature = "qualified_names")))]
            [("NaiveDateTime", None)] => {
                Ok(Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Naive)))
            }

            #[cfg(feature = "qualified_names")]
            [("flutter_rust_bridge", None), ("DartAbi", None)] => Ok(Dynamic(IrTypeDynamic)),

            [("DartAbi", None)] => Ok(Dynamic(IrTypeDynamic)),

            #[cfg(all(feature = "qualified_names"))]
            [("uuid", None), ("Uuid", None)] => Ok(Delegate(IrTypeDelegate::Uuid)),

            #[cfg(all(not(feature = "qualified_names")))]
            [("Uuid", None)] => Ok(Delegate(IrTypeDelegate::Uuid)),

            #[cfg(feature = "qualified_names")]
            [("flutter_rust_bridge", None), ("DartOpaque", None)] => {
                Ok(DartOpaque(IrTypeDartOpaque {}))
            }

            [("DartOpaque", None)] => Ok(DartOpaque(IrTypeDartOpaque {})),

            [("String", None)] => Ok(Delegate(IrTypeDelegate::String)),

            [("Backtrace", None)] => Ok(Delegate(IrTypeDelegate::Backtrace)),

            // TODO: change to "if let guard" https://github.com/rust-lang/rust/issues/51114
            [(name, None)] if matches!(parse_primitive(name), Some(..)) => {
                Ok(Primitive(parse_primitive(name).unwrap()))
            }

            [(name, None)] if self.src_structs.contains_key(&name.to_string()) => {
                let ident = IrStructIdent(name.to_string());

                if !self.parsing_or_parsed_struct_names.contains(&ident.0) {
                    self.parsing_or_parsed_struct_names.insert(ident.clone().0);
                    let api_struct = match self.parse_struct(&ident.0)? {
                        Some(ir_struct) => ir_struct,
                        None => return Ok(parse_path_type_to_unencodable(type_path, flat_vector)),
                    };
                    self.struct_pool.insert(ident.clone(), api_struct);
                }

                Ok(StructRef(IrTypeStructRef {
                    ident: ident.clone(),
                    is_exception: false,
                    // TODO rm
                    // freezed: self
                    //     .struct_pool
                    //     .get(&ident_string)
                    //     .map(IrStruct::using_freezed)
                    //     .unwrap_or(false),
                    // empty: self
                    //     .struct_pool
                    //     .get(&ident_string)
                    //     .map(IrStruct::is_empty)
                    //     .unwrap_or(false),
                }))
            }

            [(name, _)] if self.src_enums.contains_key(&name.to_string()) => {
                let ident = IrEnumIdent(name.to_string());

                if self.parsed_enums.insert(ident.clone().0) {
                    let enu = self.parse_enum(&ident.0)?;
                    self.enum_pool.insert(ident.clone(), enu);
                }

                let enum_ref = IrTypeEnumRef {
                    ident,
                    is_exception: false,
                };
                let enu = self.enum_pool.get(&ident);
                let is_struct = enu.map(IrEnum::is_struct).unwrap_or(true);
                if is_struct {
                    Ok(EnumRef(enum_ref))
                } else {
                    Ok(Delegate(IrTypeDelegate::PrimitiveEnum(
                        IrTypeDelegatePrimitiveEnum {
                            ir: enum_ref,
                            // TODO(Desdaemon): Parse #[repr] from enum
                            repr: IrTypePrimitive::I32,
                        },
                    )))
                }
            }

            // Generic types
            [("Vec", Some(Generic([Delegate(IrTypeDelegate::String)])))] => {
                Ok(Delegate(IrTypeDelegate::StringList))
            }

            [("Vec", Some(Generic([Delegate(IrTypeDelegate::Uuid)])))] => {
                Ok(Delegate(IrTypeDelegate::Uuids))
            }

            [("Vec", Some(Generic([Optional(opt)])))] => {
                if matches!(opt.inner.as_ref(), IrType::Optional(_)) {
                    bail!(
                        "Nested optionals without indirection are not allowed. {}",
                        type_path.to_token_stream()
                    );
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
            [("flutter_rust_bridge", None), ("RustOpaque", Some(Generic([Delegate(IrTypeDelegate::Array(array_delegate))])))] => {
                Ok(Delegate(IrTypeDelegate::Array(array_delegate.clone())))
            }

            [("RustOpaque", Some(Generic([Delegate(IrTypeDelegate::Array(array_delegate))])))] => {
                Ok(Delegate(IrTypeDelegate::Array(array_delegate.clone())))
            }

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

            [("Option", Some(Generic([Optional(_)])))] => Err(anyhow!(
                "Nested optionals without indirection are not supported. {}",
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
                Delegate(IrTypeDelegate::Time(..)) => IrTypeOptional::new_boxed(inner.clone()),
                OptionalList(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_) | Dynamic(_)
                | Unencodable(_) | Delegate(_) => IrTypeOptional::new(inner.clone()),
                Optional(_) => unreachable!(),
            })),

            #[cfg(all(feature = "qualified_names"))]
            [("chrono", None), ("DateTime", Some(Generic(args)))] => parse_datetime(args),

            [("DateTime", Some(Generic(args)))] => parse_datetime(args),

            _ => Ok(parse_path_type_to_unencodable(type_path, flat_vector)),
        }
    }
}

fn parse_primitive(s: &str) -> Option<IrTypePrimitive> {
    Some(match s {
        "u8" => IrTypePrimitive::U8,
        "i8" => IrTypePrimitive::I8,
        "u16" => IrTypePrimitive::U16,
        "i16" => IrTypePrimitive::I16,
        "u32" => IrTypePrimitive::U32,
        "i32" => IrTypePrimitive::I32,
        "u64" => IrTypePrimitive::U64,
        "i64" => IrTypePrimitive::I64,
        "f32" => IrTypePrimitive::F32,
        "f64" => IrTypePrimitive::F64,
        "bool" => IrTypePrimitive::Bool,
        "()" => IrTypePrimitive::Unit,
        "usize" => IrTypePrimitive::Usize,
        "isize" => IrTypePrimitive::Isize,
        _ => return None,
    })
}

fn parse_datetime(args: &[IrType]) -> anyhow::Result<IrType> {
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
                Ok(Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Utc)))
            }

            [("Utc", None)] => Ok(Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Utc))),

            #[cfg(feature = "qualified_names")]
            [("DateTime", None), ("Local", None)] => {
                Ok(Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Local)))
            }

            [("Local", None)] => Ok(Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Local))),

            _ => bail!("Invalid DateTime generic"),
        };
    }
    bail!("Invalid DateTime generic")
}

fn parse_path_type_to_unencodable(
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
