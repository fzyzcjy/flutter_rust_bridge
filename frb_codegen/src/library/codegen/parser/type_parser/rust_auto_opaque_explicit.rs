use quote::ToTokens;
use syn::Type;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::RustOpaque;
use crate::codegen::ir::ty::rust_opaque::{IrTypeRustOpaque, RustOpaqueCodecMode};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_rust_auto_opaque_explicit(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("RustAutoOpaque", [ty]) => self.parse_rust_auto_opaque_explicit(ty, None),
            ("RustAutoOpaqueNom", [ty]) => {
                self.parse_rust_auto_opaque_explicit(ty, Some(RustOpaqueCodecMode::Nom))
            }
            ("RustAutoOpaqueMoi", [ty]) => {
                self.parse_rust_auto_opaque_explicit(ty, Some(RustOpaqueCodecMode::Moi))
            }

            _ => return Ok(None),
        }))
    }

    fn parse_rust_auto_opaque_explicit(
        &mut self,
        inner: &Type,
        codec: Option<RustOpaqueCodecMode>,
    ) -> IrType {
        let inner_str = inner.to_token_stream().to_string();
        let info = self.get_or_insert_rust_auto_opaque_info(&inner_str, None, codec);

        RustOpaque(IrTypeRustOpaque {
            namespace: info.namespace,
            inner: self.create_rust_opaque_type_for_rust_auto_opaque(&inner_str),
            codec: info.codec,
            brief_name: true,
        })
    }
}
