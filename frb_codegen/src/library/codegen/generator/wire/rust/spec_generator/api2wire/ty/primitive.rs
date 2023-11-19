use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::*;

impl<'a> WireRustGeneratorApi2wireTrait for PrimitiveWireRustGenerator<'a> {
    fn intodart_type(&self, _ir_pack: &IrPack) -> String {
        match self {
            IrTypePrimitive::Unit => String::from("()"),
            _ => self.rust_api_type(),
        }
    }
}
