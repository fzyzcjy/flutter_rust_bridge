use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGenerator;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for RecordWireRustCodecDcoGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        let values = self
            .ir
            .values
            .iter()
            .map(|e| WireRustCodecDcoGenerator::new(e.clone(), self.context).intodart_type(ir_pack))
            .join(",");
        format!("({values},)")
    }
}
