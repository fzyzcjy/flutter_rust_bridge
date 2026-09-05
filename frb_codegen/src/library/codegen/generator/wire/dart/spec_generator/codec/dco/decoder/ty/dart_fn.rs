use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for DartFnWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        "throw UnimplementedError('');".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::dart_fn::{MirDartFnOutput, MirTypeDartFn};
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;

    /// Rejects DCO decoding for Dart callbacks.
    #[test]
    fn dart_function_decoder_remains_unimplemented() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let generator = DartFnWireDartCodecDcoGenerator::new(
            MirTypeDartFn {
                inputs: vec![],
                output: Box::new(MirDartFnOutput {
                    normal: MirType::Primitive(MirTypePrimitive::Unit),
                    error: MirType::Primitive(MirTypePrimitive::Unit),
                    api_fallible: false,
                }),
            },
            test_utils::context(&pack, &config),
        );

        assert_eq!(
            generator.generate_impl_decode_body(),
            "throw UnimplementedError('');"
        );
    }
}
