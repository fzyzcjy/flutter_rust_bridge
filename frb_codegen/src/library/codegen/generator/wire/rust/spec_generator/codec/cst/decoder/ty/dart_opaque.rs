use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl WireRustCodecCstGeneratorDecoderTrait for DartOpaqueWireRustCodecCstGenerator<'_> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Web => Some(
                "unsafe { flutter_rust_bridge::for_generated::cst_decode_dart_opaque(self as _) }"
                    .to_owned(),
            ),
            TargetOrCommon::Common => None,
        })
    }

    fn rust_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => "*const std::ffi::c_void",
            Target::Web => JS_VALUE,
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::dart_opaque::MirTypeDartOpaque;

    /// Emits opaque decoding and target-specific pointer versus JavaScript wire types.
    #[test]
    fn dart_opaque_decoder_covers_shared_decode_and_target_wire_types() {
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
        let generator = DartOpaqueWireRustCodecCstGenerator::new(MirTypeDartOpaque, context);

        assert_eq!(
            generator.generate_impl_decode_body().io.as_deref(),
            Some(
                "unsafe { flutter_rust_bridge::for_generated::cst_decode_dart_opaque(self as _) }"
            )
        );
        assert_eq!(
            generator.generate_impl_decode_body().web,
            generator.generate_impl_decode_body().io
        );
        assert_eq!(
            generator.rust_wire_type(Target::Io),
            "*const std::ffi::c_void"
        );
        assert_eq!(generator.rust_wire_type(Target::Web), JS_VALUE);
    }
}
