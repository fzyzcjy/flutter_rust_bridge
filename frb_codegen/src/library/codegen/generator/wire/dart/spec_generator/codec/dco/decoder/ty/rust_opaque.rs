use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for RustOpaqueWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        generalized_rust_opaque_generate_impl_decode_body(self.mir.clone().into(), self.context)
    }
}

pub(super) fn generalized_rust_opaque_generate_impl_decode_body(
    mir: MirType,
    context: WireDartCodecDcoGeneratorContext,
) -> String {
    format!(
        "return {}Impl.frbInternalDcoDecode(raw as List<dynamic>);",
        ApiDartGenerator::new(mir, context.as_api_dart_context()).dart_api_type()
    )
}
