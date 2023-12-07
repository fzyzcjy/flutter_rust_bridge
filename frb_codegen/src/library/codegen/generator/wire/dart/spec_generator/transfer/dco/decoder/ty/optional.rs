use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait for OptionalWireDartTransferDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        format!(
            "return raw == null ? null : _dco_decode_{}(raw);",
            self.ir.inner.safe_ident()
        )
    }
}
