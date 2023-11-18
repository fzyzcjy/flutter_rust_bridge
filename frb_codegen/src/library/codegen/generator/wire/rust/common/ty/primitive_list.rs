use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorCommonTrait for PrimitiveListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            match self.ir.primitive {
                IrTypePrimitive::Bool | IrTypePrimitive::Unit => JS_VALUE.into(),
                _ => format!("Box<[{}]>", self.ir.primitive.rust_api_type()),
            }
        } else {
            format!("wire_{}", self.ir.safe_ident())
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
    }
}
