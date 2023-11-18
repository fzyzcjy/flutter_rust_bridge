use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorMiscTrait for DynamicWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!("Functions cannot receive dynamic parameters.")
    }
}
