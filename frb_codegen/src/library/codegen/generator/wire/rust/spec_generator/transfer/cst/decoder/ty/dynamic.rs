use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;

impl<'a> WireRustTransferCstGeneratorDecoderTrait for DynamicWireRustTransferCstGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        // Functions cannot receive dynamic parameters
        "UNREACHABLE_RUST_WIRE_TYPE".into()
    }
}
