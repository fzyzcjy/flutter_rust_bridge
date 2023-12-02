use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for DynamicWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        // Functions cannot receive dynamic parameters
        "UNREACHABLE_RUST_WIRE_TYPE".into()
    }
}
