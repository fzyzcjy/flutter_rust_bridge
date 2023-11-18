use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for UnencodableWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        "".to_string()
    }
}
