use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::codec::{BaseCodecEntrypointTrait, CodecMode};
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::entrypoint::CstWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::entrypoint::DcoWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::entrypoint::SseWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::ir::func::IrFunc;
use crate::codegen_codec_structs;
use enum_dispatch::enum_dispatch;
use serde::Serialize;

codegen_codec_structs!(WireRustCodecEntrypoint);

pub(crate) trait WireRustCodecEntrypointTrait<'a>:
    BaseCodecEntrypointTrait<WireRustGeneratorContext<'a>, WireRustCodecOutputSpec>
{
    fn generate_func_params(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>>;

    fn generate_func_call_decode(&self, func: &IrFunc, context: WireRustGeneratorContext)
        -> String;
}

#[derive(Clone, Serialize)]
pub(crate) struct WireRustCodecOutputSpec {}
