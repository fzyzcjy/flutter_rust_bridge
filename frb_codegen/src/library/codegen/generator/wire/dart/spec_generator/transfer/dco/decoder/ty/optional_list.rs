use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait for OptionalListWireDartTransferDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        format!(
            "return mapNonNull(raw as List<dynamic>, _dco_decode_{});",
            self.ir.inner.safe_ident()
        )
    }
}
