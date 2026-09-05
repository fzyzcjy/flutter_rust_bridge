use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for RustOpaqueWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        generalized_rust_opaque_generate_impl_decode_body(self.mir.clone().into(), self.context)
    }
}

pub(super) fn generalized_rust_opaque_generate_impl_decode_body(
    mir: MirType,
    context: WireDartCodecDcoGeneratorContext,
) -> String {
    format!(
        "return {}Impl.frbInternalDcoDecode(dcoDecodeList(raw));",
        ApiDartGenerator::new(mir, context.as_api_dart_context()).dart_api_type()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::utils::namespace::Namespace;

    fn opaque() -> MirType {
        MirType::RustOpaque(MirTypeRustOpaque {
            namespace: Namespace::default(),
            inner: MirRustOpaqueInner(MirLifetimeAwareType::new("crate::api::Handle".into())),
            codec: RustOpaqueCodecMode::Nom,
            dart_api_type: Some("Handle".into()),
            brief_name: false,
        })
    }

    /// Routes Rust opaque raw values through the generated implementation decoder.
    #[test]
    fn rust_opaque_decoder_uses_api_type_implementation() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let output = generalized_rust_opaque_generate_impl_decode_body(
            opaque(),
            test_utils::context(&pack, &config),
        );

        assert_eq!(
            output,
            "return HandleImpl.frbInternalDcoDecode(dcoDecodeList(raw));"
        );
    }
}
