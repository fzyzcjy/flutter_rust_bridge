use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for DynamicWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        "return raw;".into()
    }
}
