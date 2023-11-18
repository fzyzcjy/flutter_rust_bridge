use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for OptionalListWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        format!(
            "return mapNonNull(raw as List<dynamic>, _wire2api_{});",
            self.ir.inner.safe_ident()
        )
    }
}
