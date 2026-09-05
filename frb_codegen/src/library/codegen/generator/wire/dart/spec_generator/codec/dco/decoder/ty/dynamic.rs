use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for DynamicWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        "return raw;".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::dynamic::MirTypeDynamic;

    /// Returns dynamic Dart ABI values without a conversion.
    #[test]
    fn dynamic_decoder_preserves_raw_value() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let generator = DynamicWireDartCodecDcoGenerator::new(
            MirTypeDynamic,
            test_utils::context(&pack, &config),
        );

        assert_eq!(generator.generate_impl_decode_body(), "return raw;");
    }
}
