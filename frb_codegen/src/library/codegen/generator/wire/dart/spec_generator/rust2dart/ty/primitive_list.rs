use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::misc::gen_wire2api_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartGeneratorRust2DartTrait for PrimitiveListWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        match &self.ir.primitive {
            IrTypePrimitive::I64 => "return Int64List.from(raw);".into(),
            IrTypePrimitive::U64 => "return Uint64List.from(raw);".into(),
            _ => gen_wire2api_simple_type_cast(self.ir.clone().into(), self.context),
        }
    }
}
