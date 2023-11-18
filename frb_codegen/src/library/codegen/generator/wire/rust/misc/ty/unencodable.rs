use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorMiscTrait for UnencodableWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!()
    }
}
