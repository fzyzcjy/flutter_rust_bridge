use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for DartOpaqueWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        "return generalizedFrbRustBinding.getDartObject(raw);".into()
    }
}
