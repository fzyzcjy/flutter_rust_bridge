use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::BaseCodecEntrypointTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypointTrait, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;

pub(crate) struct SseWireRustCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireRustGeneratorContext<'_>, WireRustCodecOutputSpec>
    for SseWireRustCodecEntrypoint
{
    fn generate_encode(
        &self,
        context: WireRustGeneratorContext,
        types: &[IrType],
    ) -> Option<WireRustCodecOutputSpec> {
        Some(super::encoder::generate(
            context.as_wire_rust_codec_sse_context(),
            types,
        ))
    }

    fn generate_decode(
        &self,
        context: WireRustGeneratorContext,
        types: &[IrType],
    ) -> Option<WireRustCodecOutputSpec> {
        Some(super::decoder::generate(
            context.as_wire_rust_codec_sse_context(),
            types,
        ))
    }
}

impl WireRustCodecEntrypointTrait<'_> for SseWireRustCodecEntrypoint {
    fn generate_func_params(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        todo!()
    }

    fn generate_func_call_decode(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> String {
        todo!()
    }
}
