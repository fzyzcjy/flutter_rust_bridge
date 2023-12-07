use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for DartFnWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        "return _dco_decode_DartOpaque(raw);".into()
    }
}
