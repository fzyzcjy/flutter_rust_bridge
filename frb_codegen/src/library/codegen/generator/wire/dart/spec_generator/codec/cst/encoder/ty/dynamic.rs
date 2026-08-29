use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;

impl WireDartCodecCstGeneratorEncoderTrait for DynamicWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc::default()
    }

    fn dart_wire_type(&self, _target: Target) -> String {
        // Functions cannot receive dynamic parameters
        "UNREACHABLE_DART_WIRE_TYPE".into()
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::ty::dynamic::MirTypeDynamic;

    /// Leaves unsupported dynamic parameters without an encoder body or wire type.
    #[test]
    fn dynamic_encoder_has_no_body_and_unreachable_wire_type() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(false);
        let wire_rust_config = test_utils::wire_rust_config(false);
        let generator = DynamicWireDartCodecCstGenerator::new(
            MirTypeDynamic,
            test_utils::context(
                &pack,
                &wire_dart_config,
                &wire_rust_config,
                &api_dart_config,
            ),
        );

        assert_eq!(generator.generate_encode_func_body(), Acc::default());
        assert_eq!(
            generator.dart_wire_type(Target::Io),
            "UNREACHABLE_DART_WIRE_TYPE"
        );
    }
}
