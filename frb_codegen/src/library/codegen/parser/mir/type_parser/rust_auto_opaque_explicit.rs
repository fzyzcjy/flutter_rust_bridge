use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateRustAutoOpaqueExplicit,
};
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_rust_auto_opaque_explicit(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
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
    ) -> anyhow::Result<MirType> {
        let (ans_raw, ans_inner) =
            self.parse_type_rust_auto_opaque_common(inner.clone(), None, codec)?;
        Ok(MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(
            MirTypeDelegateRustAutoOpaqueExplicit {
                raw: ans_raw,
                inner: ans_inner,
            },
        )))
    }
}
