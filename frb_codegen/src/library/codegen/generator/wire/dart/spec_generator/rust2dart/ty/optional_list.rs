use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorWire2apiTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorWire2apiTrait for OptionalListWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        format!(
            "return mapNonNull(raw as List<dynamic>, _wire2api_{});",
            self.ir.inner.safe_ident()
        )
    }
}
