use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for DartOpaqueWireRustCodecCstGenerator<'a> {
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
