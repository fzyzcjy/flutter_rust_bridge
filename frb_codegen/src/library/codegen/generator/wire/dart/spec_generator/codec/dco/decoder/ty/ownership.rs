use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for OwnershipWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        "".to_string()
    }
}
