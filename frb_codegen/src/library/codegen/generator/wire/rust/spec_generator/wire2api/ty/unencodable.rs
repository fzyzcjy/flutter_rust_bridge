use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for UnencodableWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!()
    }
}
