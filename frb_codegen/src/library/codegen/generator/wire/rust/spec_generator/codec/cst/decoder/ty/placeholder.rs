use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for PlaceholderWireRustCodecCstGenerator<'a> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc::distribute(None)
    }

    fn rust_wire_type(&self, target: Target) -> String {
        "NOT_USED".to_owned()
    }
}
