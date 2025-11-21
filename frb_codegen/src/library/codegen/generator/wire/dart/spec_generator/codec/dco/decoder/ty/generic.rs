use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for GenericWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        // Generic types should not appear in generated code
        "unreachable!()".into()
    }
}

