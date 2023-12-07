use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for PrimitiveWireRustCodecDcoGenerator<'a> {
    fn intodart_type(&self, _ir_pack: &IrPack) -> String {
        match self.ir {
            IrTypePrimitive::Unit => String::from("()"),
            _ => self.ir.rust_api_type(),
        }
    }
}
