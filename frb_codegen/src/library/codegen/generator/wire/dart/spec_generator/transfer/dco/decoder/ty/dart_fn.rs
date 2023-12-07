use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for DartFnWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        "return _dco_decode_DartOpaque(raw);".into()
    }
}
