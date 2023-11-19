use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::*;

impl<'a> WireRustGeneratorApi2wireTrait for RecordWireRustGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        let values = self
            .values
            .iter()
            .map(|e| e.intodart_type(ir_pack))
            .collect_vec()
            .join(",");
        format!("({values},)")
    }
}
