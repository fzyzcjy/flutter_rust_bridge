use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::wire2api::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for DartOpaqueWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        "return wire.get_dart_object(raw);".into()
    }
}
