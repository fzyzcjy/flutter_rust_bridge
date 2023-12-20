use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for OwnershipWireRustCodecCstGenerator<'a> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!()
    }
}
