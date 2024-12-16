use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for DartOpaqueWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        "return decodeDartOpaque(raw, generalizedFrbRustBinding);".into()
    }
}
