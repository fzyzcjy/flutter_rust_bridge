use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for OptionalWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        format!(
            "return raw == null ? null : dco_decode_{}(raw);",
            self.ir.inner.safe_ident()
        )
    }
}
