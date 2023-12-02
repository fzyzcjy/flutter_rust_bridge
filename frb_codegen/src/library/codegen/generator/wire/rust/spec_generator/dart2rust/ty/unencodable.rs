use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorWire2apiTrait;

impl<'a> WireRustGeneratorWire2apiTrait for UnencodableWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!()
    }
}
