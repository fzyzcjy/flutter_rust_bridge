use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait
    for RustAutoOpaqueWireDartTransferDcoGenerator<'a>
{
    fn generate_impl_wire2api_body(&self) -> String {
        generalized_rust_opaque_generate_impl_wire2api_body(self.ir.clone().into(), self.context)
    }
}
