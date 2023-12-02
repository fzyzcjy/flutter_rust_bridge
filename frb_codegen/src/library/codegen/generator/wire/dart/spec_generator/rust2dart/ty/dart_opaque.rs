use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;

impl<'a> WireDartGeneratorRust2DartTrait for DartOpaqueWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        "return generalizedFrbRustBinding.getDartObject(raw);".into()
    }
}
