use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::rust_opaque::generalized_rust_opaque_generate_impl_decode_body;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for RustAutoOpaqueImplicitWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        generalized_rust_opaque_generate_impl_decode_body(self.mir.clone().into(), self.context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::func::OwnershipMode;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
        MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
    };
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::utils::namespace::Namespace;

    /// Reuses the Rust opaque implementation decoder for implicit opaque values.
    #[test]
    fn implicit_rust_opaque_decoder_delegates_to_generalized_decoder() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let inner = MirTypeRustOpaque {
            namespace: Namespace::default(),
            inner: MirRustOpaqueInner(MirLifetimeAwareType::new("crate::api::Handle".into())),
            codec: RustOpaqueCodecMode::Nom,
            dart_api_type: Some("Handle".into()),
            brief_name: false,
        };
        let generator = RustAutoOpaqueImplicitWireDartCodecDcoGenerator::new(
            MirTypeRustAutoOpaqueImplicit {
                ownership_mode: OwnershipMode::Owned,
                inner,
                raw: MirRustAutoOpaqueRaw {
                    string: MirLifetimeAwareType::new("crate::api::Handle".into()),
                    segments: vec![],
                },
                reason: None,
                ignore: false,
            },
            test_utils::context(&pack, &config),
        );

        assert_eq!(
            generator.generate_impl_decode_body(),
            "return HandleImpl.frbInternalDcoDecode(raw as List<dynamic>);"
        );
    }
}
