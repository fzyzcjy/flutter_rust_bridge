use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateRustAutoOpaqueExplicit};
use crate::codegen::ir::ty::rust_opaque::{IrTypeRustOpaque, RustOpaqueCodecMode};
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use quote::ToTokens;
use syn::Type;
use crate::codegen::ir::ty::IrType::RustAutoOpaqueImplicit;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_rust_auto_opaque_explicit(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("RustAutoOpaque", [ty]) => self.parse_rust_auto_opaque_explicit(ty, None)?,
            ("RustAutoOpaqueNom", [ty]) => {
                self.parse_rust_auto_opaque_explicit(ty, Some(RustOpaqueCodecMode::Nom))?
            }
            ("RustAutoOpaqueMoi", [ty]) => {
                self.parse_rust_auto_opaque_explicit(ty, Some(RustOpaqueCodecMode::Moi))?
            }

            _ => return Ok(None),
        }))
    }

    fn parse_rust_auto_opaque_explicit(
        &mut self,
        inner: &Type,
        codec: Option<RustOpaqueCodecMode>,
    ) -> anyhow::Result<IrType> {
        let (ans_raw, ans_inner) =
            self.parse_type_rust_auto_opaque_common(inner.clone(), None, codec)?;
        Ok(IrType::Delegate(IrTypeDelegate::RustAutoOpaqueExplicit(
            IrTypeDelegateRustAutoOpaqueExplicit {
                raw: ans_raw,
                inner: ans_inner,
            },
        )))
    }
}
