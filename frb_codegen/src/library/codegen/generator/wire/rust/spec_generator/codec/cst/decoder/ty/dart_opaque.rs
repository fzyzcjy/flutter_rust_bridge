use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::rust_opaque::dart_opaque_or_generalized_rust_opaque_rust_wire_type;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::misc::consts::HANDLER_NAME;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for DartOpaqueWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc::new(|target| {
            match target {
                TargetOrCommon::Io => Some(
                    "unsafe { flutter_rust_bridge::for_generated::cst_decode_dart_opaque(self) }"
                        .to_owned(),
                ),
                TargetOrCommon::Wasm => Some(
                    format!("unsafe {{ flutter_rust_bridge::for_generated::cst_decode_dart_opaque(&*{HANDLER_NAME}, self) }}"),
                ),
                TargetOrCommon::Common => None,
            }
        })
    }

    fn rust_wire_type(&self, target: Target) -> String {
        dart_opaque_or_generalized_rust_opaque_rust_wire_type(target)
    }
}
