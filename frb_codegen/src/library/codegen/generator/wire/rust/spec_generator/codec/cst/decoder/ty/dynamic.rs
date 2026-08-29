use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl WireRustCodecCstGeneratorDecoderTrait for DynamicWireRustCodecCstGenerator<'_> {
    fn rust_wire_type(&self, _target: Target) -> String {
        // Functions cannot receive dynamic parameters
        "UNREACHABLE_RUST_WIRE_TYPE".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::dynamic::MirTypeDynamic;

    /// Rejects dynamic function parameters with the explicit sentinel wire type.
    #[test]
    fn dynamic_decoder_uses_unreachable_wire_type_for_both_targets() {
        let pack = test_utils::pack();
        let wire_dart_config = test_utils::wire_dart_config(true);
        let wire_rust_config = test_utils::wire_rust_config(true);
        let api_dart_config = test_utils::api_dart_config();
        let dart_context = test_utils::context(
            &pack,
            &wire_dart_config,
            &wire_rust_config,
            &api_dart_config,
        );
        let context = dart_context.as_wire_rust_context();
        let generator = DynamicWireRustCodecCstGenerator::new(MirTypeDynamic, context);

        assert_eq!(
            generator.rust_wire_type(Target::Io),
            "UNREACHABLE_RUST_WIRE_TYPE"
        );
        assert_eq!(
            generator.rust_wire_type(Target::Web),
            "UNREACHABLE_RUST_WIRE_TYPE"
        );
    }
}
