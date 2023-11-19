use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::wire2api::misc::gen_wire2api_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::wire2api::ty::WireDartGeneratorWire2apiTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartGeneratorWire2apiTrait for PrimitiveWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        match self.ir {
            IrTypePrimitive::Unit => "return;".to_owned(),
            IrTypePrimitive::I64 | IrTypePrimitive::U64 | IrTypePrimitive::Usize => {
                "return castInt(raw);".to_owned()
            }
            _ => gen_wire2api_simple_type_cast(self.ir.clone().into(), self.context),
        }
    }
}
