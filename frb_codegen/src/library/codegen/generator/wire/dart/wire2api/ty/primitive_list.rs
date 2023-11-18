use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for PrimitiveListWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        match &self.ir.primitive {
            IrTypePrimitive::I64 => "return Int64List.from(raw);".into(),
            IrTypePrimitive::U64 => "return Uint64List.from(raw);".into(),
            _ => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
        }
    }
}
