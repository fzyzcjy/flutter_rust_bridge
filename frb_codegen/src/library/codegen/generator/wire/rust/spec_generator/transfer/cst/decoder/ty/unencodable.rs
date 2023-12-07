use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for UnencodableWireRustCodecCstGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!()
    }
}
