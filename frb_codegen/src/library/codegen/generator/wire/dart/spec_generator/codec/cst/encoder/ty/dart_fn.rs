use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl WireDartCodecCstGeneratorEncoderTrait for DartFnWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::new_common(Some(format!(
            "return cst_encode_DartOpaque(encode_{}(raw));",
            self.mir.safe_ident(),
        )))
    }

    fn dart_wire_type(&self, target: Target) -> String {
        WireDartCodecCstGenerator::new(self.mir.get_delegate(), self.context).dart_wire_type(target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::ty::dart_fn::{MirDartFnOutput, MirTypeDartFn};
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;

    /// Wraps encoded Dart callbacks as Dart opaque values using their stable identifier.
    #[test]
    fn dart_function_encoder_emits_callback_and_opaque_encoding_call() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(false);
        let wire_rust_config = test_utils::wire_rust_config(false);
        let mir = MirTypeDartFn {
            inputs: vec![MirType::Primitive(MirTypePrimitive::I32)],
            output: Box::new(MirDartFnOutput {
                normal: MirType::Primitive(MirTypePrimitive::Bool),
                error: MirType::Primitive(MirTypePrimitive::Unit),
                api_fallible: false,
            }),
        };
        let expected_ident = mir.safe_ident();
        let generator = DartFnWireDartCodecCstGenerator::new(
            mir,
            test_utils::context(
                &pack,
                &wire_dart_config,
                &wire_rust_config,
                &api_dart_config,
            ),
        );

        let expected = format!("return cst_encode_DartOpaque(encode_{expected_ident}(raw));");
        assert_eq!(
            generator.generate_encode_func_body().common.as_deref(),
            Some(expected.as_str())
        );
    }
}
