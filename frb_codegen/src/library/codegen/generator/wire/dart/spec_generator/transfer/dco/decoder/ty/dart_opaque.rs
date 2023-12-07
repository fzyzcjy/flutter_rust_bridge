use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for DartOpaqueWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        "return dcoDecodeDartOpaque(raw, generalizedFrbRustBinding);".into()
    }
}
