use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::*;

impl<'a> WireRustGeneratorApi2wireTrait for GeneralListWireRustGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        format!("Vec<{}>", self.inner.intodart_type(ir_pack))
    }
}
