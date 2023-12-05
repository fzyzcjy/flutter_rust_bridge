use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::unencodable::{
    parse_path_type_to_unencodable, splay_segments,
};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use quote::ToTokens;
use syn::{Path, QSelf, TypePath};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path(&mut self, type_path: &TypePath) -> anyhow::Result<IrType> {
        match &type_path {
            TypePath { qself: None, path } => self.parse_type_path_core(type_path, path),
            TypePath {
                qself: Some(QSelf { ty, .. }),
                ..
            } => bail!(
                "qself \"<{}>\" in \"{}\", and all qself syntax, is unsupported",
                ty.to_token_stream(),
                type_path.to_token_stream()
            ),
        }
    }

    fn parse_type_path_core(
        &mut self,
        type_path: &TypePath,
        path: &Path,
    ) -> anyhow::Result<IrType> {
        let segments = self.extract_path_data(path)?;
        let splayed_segments = splay_segments(&segments);

        if let Some(last_segment) = splayed_segments.last() {
            if let Some(ans) = self.parse_type_path_data_primitive(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) =
                self.parse_type_path_data_struct(type_path, &splayed_segments, last_segment)?
            {
                return Ok(ans);
            }
            if let Some(ans) =
                self.parse_type_path_data_enum(type_path, &splayed_segments, last_segment)?
            {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_rust_opaque(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_concrete(last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_vec(type_path, last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_optional(type_path, last_segment)? {
                return Ok(ans);
            }
            if let Some(ans) = self.parse_type_path_data_dart_fn(last_segment)? {
                return Ok(ans);
            }
        }

        Ok(parse_path_type_to_unencodable(type_path, &splayed_segments))
    }
}
