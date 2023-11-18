use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for RustOpaqueWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        let bridge = if self.context.config.use_bridge_in_method {
            "this"
        } else {
            ""
        };
        format!(
            "return {0}.fromRaw(raw[0], raw[1], {bridge});",
            self.ir.dart_api_type()
        )
    }
}
