use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for UnencodableWireDartCodecDcoGenerator<'a> {
    // frb-coverage:ignore-start
    fn generate_impl_decode_body(&self) -> String {
        unreachable!()
    }
    // frb-coverage:ignore-end
}
