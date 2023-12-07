use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait for OptionalListWireDartTransferDcoGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        format!(
            "return mapNonNull(raw as List<dynamic>, _wire2api_{});",
            self.ir.inner.safe_ident()
        )
    }
}
