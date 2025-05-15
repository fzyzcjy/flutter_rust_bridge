use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for DynamicWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        "return raw;".into()
    }
}
