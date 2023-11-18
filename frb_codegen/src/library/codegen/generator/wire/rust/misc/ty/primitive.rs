use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorMiscTrait for PrimitiveWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        self.ir.rust_api_type()
    }
}
