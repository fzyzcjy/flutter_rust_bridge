use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for DartFnWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        "throw UnimplementedError('');".into()
    }
}
