use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for DynamicWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!("Functions cannot receive dynamic parameters.")
    }
}
