use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl WireRustCodecCstGeneratorDecoderTrait for DartFnWireRustCodecCstGenerator<'_> {
    fn generate_wire_func_param_api_type(&self) -> Option<String> {
        Some(self.mir.get_delegate().rust_api_type())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        WireRustCodecCstGenerator::new(self.mir.get_delegate(), self.context).rust_wire_type(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::dart_fn::{MirDartFnOutput, MirTypeDartFn};
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;

    /// Delegates Dart function parameters to the Dart opaque CST wire representation.
    #[test]
    fn dart_function_decoder_uses_dart_opaque_api_and_wire_types() {
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
        let generator = DartFnWireRustCodecCstGenerator::new(
            MirTypeDartFn {
                inputs: vec![],
                output: Box::new(MirDartFnOutput {
                    normal: MirType::Primitive(MirTypePrimitive::Unit),
                    error: MirType::Primitive(MirTypePrimitive::Unit),
                    api_fallible: false,
                }),
            },
            context,
        );

        assert_eq!(
            generator.generate_wire_func_param_api_type().as_deref(),
            Some("flutter_rust_bridge::DartOpaque")
        );
        assert_eq!(
            generator.rust_wire_type(Target::Io),
            "*const std::ffi::c_void"
        );
        assert_eq!(
            generator.rust_wire_type(Target::Web),
            "flutter_rust_bridge::for_generated::wasm_bindgen::JsValue"
        );
    }
}
