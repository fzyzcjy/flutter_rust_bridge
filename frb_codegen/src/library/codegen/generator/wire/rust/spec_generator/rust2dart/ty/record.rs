use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

impl<'a> WireRustGeneratorApi2wireTrait for RecordWireRustGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        let values = self
            .ir
            .values
            .iter()
            .map(|e| WireRustGenerator::new(e.clone(), self.context).intodart_type(ir_pack))
            .join(",");
        format!("({values},)")
    }
}
