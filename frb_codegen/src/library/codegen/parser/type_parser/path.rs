use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::enumeration::{IrEnumIdent, IrTypeEnumRef};
use crate::codegen::ir::ty::optional::IrTypeOptional;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::structure::{IrStructIdent, IrTypeStructRef};
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
        use ArgsRefs::*;

        let segments = self.extract_path_data(path)?;
        let splayed_segments: &[(&str, Option<ArgsRefs>)] = &segments.splay()[..];

        if let Some(ans) = self.parse_type_path_data_primitive(splayed_segments)? {
            return Ok(ans);
        }
        if let Some(ans) = self.parse_type_path_data_concrete(splayed_segments)? {
            return Ok(ans);
        }
        if let Some(ans) = self.parse_type_path_data_vec(type_path, splayed_segments)? {
            return Ok(ans);
        }

        match splayed_segments {
            [(name, None)] if self.src_structs.contains_key(&name.to_string()) => {
                let ident = IrStructIdent(name.to_string());

                if !self.parsing_or_parsed_struct_names.contains(&ident.0) {
                    self.parsing_or_parsed_struct_names.insert(ident.clone().0);
                    let api_struct = match self.parse_struct(&ident.0)? {
                        Some(ir_struct) => ir_struct,
                        None => {
                            return Ok(parse_path_type_to_unencodable(type_path, segments.splay()))
                        }
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
                    ident: ident.clone(),
                    is_exception: false,
                };
                let enu = self.enum_pool.get(&ident);
                let is_struct = enu.map(|e| e.is_struct).unwrap_or(true);
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

            [("flutter_rust_bridge", None), (
                "ZeroCopyBuffer",
                Some(Generic([PrimitiveList(IrTypePrimitiveList { primitive })])),
            )] => Ok(Delegate(IrTypeDelegate::ZeroCopyBufferVecPrimitive(
                primitive.clone(),
            ))),

            [("flutter_rust_bridge", None), ("RustOpaque", Some(Generic([Delegate(IrTypeDelegate::Array(array_delegate))])))] => {
                Ok(Delegate(IrTypeDelegate::Array(array_delegate.clone())))
            }

            [("flutter_rust_bridge", None), ("RustOpaque", Some(Generic([Primitive(IrTypePrimitive::Unit)])))] => {
                Ok(RustOpaque(IrTypeRustOpaque::new(Primitive(
                    IrTypePrimitive::Unit,
                ))))
            }

            [("flutter_rust_bridge", None), ("RustOpaque", Some(Generic([ty])))] => {
                Ok(RustOpaque(IrTypeRustOpaque::new(ty.clone())))
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
                    IrTypeOptional::new_with_boxed_wrapper(inner.clone())
                }
                Delegate(IrTypeDelegate::Time(..)) => {
                    IrTypeOptional::new_with_boxed_wrapper(inner.clone())
                }
                OptionalList(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_) | Dynamic(_)
                | Unencodable(_) | Delegate(_) => IrTypeOptional::new(inner.clone()),
                Optional(_) => unreachable!(),
            })),

            _ => Ok(parse_path_type_to_unencodable(type_path, segments.splay())),
        }
    }
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
