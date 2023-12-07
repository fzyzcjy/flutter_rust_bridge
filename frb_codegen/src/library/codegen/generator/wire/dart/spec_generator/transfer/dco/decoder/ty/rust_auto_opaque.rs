use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::rust_opaque::generalized_rust_opaque_generate_impl_decode_body;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait
    for RustAutoOpaqueWireDartTransferDcoGenerator<'a>
{
    fn generate_impl_decode_body(&self) -> String {
        generalized_rust_opaque_generate_impl_decode_body(self.ir.clone().into(), self.context)
    }
}
