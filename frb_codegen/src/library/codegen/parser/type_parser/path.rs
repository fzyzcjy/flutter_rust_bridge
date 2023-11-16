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
use crate::codegen::parser::type_parser::unencodable::ArgsRefs;
use crate::codegen::parser::type_parser::unencodable::{
    parse_path_type_to_unencodable, splay_segments,
};
use crate::codegen::parser::type_parser::TypeParser;
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
        let splayed_segments = splay_segments(&segments);

        if let Some(ans) = self.parse_type_path_data_primitive(&splayed_segments)? {
            return Ok(ans);
        }
        if let Some(ans) = self.parse_type_path_data_struct(type_path, &splayed_segments)? {
            return Ok(ans);
        }
        if let Some(ans) = self.parse_type_path_data_enum(&splayed_segments)? {
            return Ok(ans);
        }
        if let Some(ans) = self.parse_type_path_data_concrete(&splayed_segments)? {
            return Ok(ans);
        }
        if let Some(ans) = self.parse_type_path_data_vec(type_path, &splayed_segments)? {
            return Ok(ans);
        }
        if let Some(ans) = self.parse_type_path_data_optional(type_path, &splayed_segments)? {
            return Ok(ans);
        }

        Ok(parse_path_type_to_unencodable(type_path, &splayed_segments))
    }
}
