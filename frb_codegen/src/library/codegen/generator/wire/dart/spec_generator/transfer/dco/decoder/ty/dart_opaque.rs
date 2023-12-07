use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait for DartOpaqueWireDartTransferDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        "return wire2apiDartOpaque(raw, generalizedFrbRustBinding);".into()
    }
}
