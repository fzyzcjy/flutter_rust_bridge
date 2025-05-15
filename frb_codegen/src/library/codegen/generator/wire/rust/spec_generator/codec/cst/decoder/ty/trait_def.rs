use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl WireRustCodecCstGeneratorDecoderTrait for TraitDefWireRustCodecCstGenerator<'_> {
    fn rust_wire_type(&self, _target: Target) -> String {
        "UNREACHABLE_RUST_WIRE_TYPE".into()
    }
}
