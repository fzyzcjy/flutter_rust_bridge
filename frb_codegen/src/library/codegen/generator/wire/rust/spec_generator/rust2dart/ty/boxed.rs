use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::pack::IrPack;

impl<'a> WireRustGeneratorRust2DartTrait for BoxedWireRustGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        WireRustGenerator::new(self.ir.inner.clone(), self.context).intodart_type(ir_pack)
    }

    fn generate_access_object_core(&self, obj: String) -> String {
        format!("(*{obj})")
    }
}
