use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::pack::IrPack;

impl<'a> WireRustGeneratorApi2wireTrait for BoxedWireRustGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> NamespacedName {
        WireRustGenerator::new(self.ir.inner.clone(), self.context).intodart_type(ir_pack)
    }

    fn generate_access_object_core(&self, obj: String) -> String {
        format!("(*{obj})")
    }
}
