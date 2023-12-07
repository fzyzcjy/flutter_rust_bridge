use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGenerator;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::encoder::ty::WireRustTransferDcoGeneratorEncoderTrait;
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

impl<'a> WireRustTransferDcoGeneratorEncoderTrait for RecordWireRustTransferDcoGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        let values = self
            .ir
            .values
            .iter()
            .map(|e| {
                WireRustTransferDcoGenerator::new(e.clone(), self.context).intodart_type(ir_pack)
            })
            .join(",");
        format!("({values},)")
    }
}
