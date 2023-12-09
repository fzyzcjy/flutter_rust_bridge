use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for RustOpaqueWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        generalized_rust_opaque_generate_impl_decode_body(self.ir.clone().into(), self.context)
    }
}

pub(super) fn generalized_rust_opaque_generate_impl_decode_body(
    ir: IrType,
    context: WireDartCodecDcoGeneratorContext,
) -> String {
    format!(
        "return {}.dcoDecode(raw);",
        ApiDartGenerator::new(ir, context.as_api_dart_context()).dart_api_type()
    )
}
