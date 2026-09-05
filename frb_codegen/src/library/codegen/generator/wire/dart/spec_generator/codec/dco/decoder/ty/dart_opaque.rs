use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for DartOpaqueWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        "return decodeDartOpaque(raw, generalizedFrbRustBinding);".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::dart_opaque::MirTypeDartOpaque;

    /// Routes opaque Dart values through the generalized binding decoder.
    #[test]
    fn dart_opaque_decoder_uses_generalized_binding() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let generator = DartOpaqueWireDartCodecDcoGenerator::new(
            MirTypeDartOpaque,
            test_utils::context(&pack, &config),
        );

        assert_eq!(
            generator.generate_impl_decode_body(),
            "return decodeDartOpaque(raw, generalizedFrbRustBinding);"
        );
    }
}
