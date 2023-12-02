use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorRust2DartTrait for PrimitiveWireRustGenerator<'a> {
    fn intodart_type(&self, _ir_pack: &IrPack) -> String {
        match self.ir {
            IrTypePrimitive::Unit => String::from("()"),
            _ => self.ir.rust_api_type(),
        }
    }
}
