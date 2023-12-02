use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;

impl<'a> WireDartGeneratorRust2DartTrait for UnencodableWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        "".to_string()
    }
}
