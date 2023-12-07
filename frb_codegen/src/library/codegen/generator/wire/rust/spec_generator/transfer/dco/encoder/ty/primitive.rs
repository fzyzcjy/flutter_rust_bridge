use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::encoder::ty::WireRustTransferDcoGeneratorEncoderTrait;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustTransferDcoGeneratorEncoderTrait for PrimitiveWireRustTransferDcoGenerator<'a> {
    fn intodart_type(&self, _ir_pack: &IrPack) -> String {
        match self.ir {
            IrTypePrimitive::Unit => String::from("()"),
            _ => self.ir.rust_api_type(),
        }
    }
}
