use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorWire2apiTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorWire2apiTrait for GeneralListWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        format!(
            "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
            self.ir.inner.safe_ident()
        )
    }
}
