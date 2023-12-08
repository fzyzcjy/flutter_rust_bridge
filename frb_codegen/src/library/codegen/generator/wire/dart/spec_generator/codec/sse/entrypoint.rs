use crate::codegen::generator::misc::codec::BaseCodecEntrypointTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;

pub(crate) struct SseWireDartCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for SseWireDartCodecEntrypoint
{
    fn generate_encode(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
    ) -> Option<WireDartCodecOutputSpec> {
        Some(super::encoder::generate(
            context.as_wire_dart_codec_sse_context(),
            types,
        ))
    }

    fn generate_decode(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
    ) -> Option<WireDartCodecOutputSpec> {
        Some(super::decoder::generate(
            context.as_wire_dart_codec_sse_context(),
            types,
        ))
    }
}

impl WireDartCodecEntrypointTrait<'_> for SseWireDartCodecEntrypoint {
    fn generate_dart2rust_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String> {
        todo!()
    }

    fn generate_dart2rust_func_wire_param_list(
        &self,
        func: &IrFunc,
        num_prepare_args: usize,
    ) -> Vec<String> {
        todo!()
    }

    fn generate_rust2dart_codec_object(&self, func: &IrFunc) -> String {
        todo!()
    }
}
