use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait for DynamicWireDartTransferDcoGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        "return raw;".into()
    }
}
