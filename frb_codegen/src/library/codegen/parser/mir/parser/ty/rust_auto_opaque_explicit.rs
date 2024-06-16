use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegateRustAutoOpaqueExplicit,
};
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::utils::namespace::Namespace;
use syn::Type;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_rust_auto_opaque_explicit(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        Ok(Some(match last_segment {
            ("RustAutoOpaque", [ty]) => self.parse_rust_auto_opaque_explicit(ty, None, None)?,
            ("RustAutoOpaqueNom", [ty]) => {
                self.parse_rust_auto_opaque_explicit(ty, None, Some(RustOpaqueCodecMode::Nom))?
            }
            ("RustAutoOpaqueMoi", [ty]) => {
                self.parse_rust_auto_opaque_explicit(ty, None, Some(RustOpaqueCodecMode::Moi))?
            }

            _ => return Ok(None),
        }))
    }

    fn parse_rust_auto_opaque_explicit(
        &mut self,
        inner: &Type,
        namespace: Option<Namespace>,
        codec: Option<RustOpaqueCodecMode>,
    ) -> anyhow::Result<MirType> {
        Ok(MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(
            self.parse_rust_auto_opaque_explicit_typed(inner, namespace, codec, None)?,
        )))
    }

    pub(crate) fn parse_rust_auto_opaque_explicit_typed(
        &mut self,
        inner: &Type,
        namespace: Option<Namespace>,
        codec: Option<RustOpaqueCodecMode>,
        dart_api_type: Option<String>,
    ) -> anyhow::Result<MirTypeDelegateRustAutoOpaqueExplicit> {
        let (ans_raw, ans_inner) = self.parse_type_rust_auto_opaque_common(
            inner.clone(),
            namespace,
            codec,
            dart_api_type,
        )?;
        Ok(MirTypeDelegateRustAutoOpaqueExplicit {
            raw: ans_raw,
            inner: ans_inner,
        })
    }
}
