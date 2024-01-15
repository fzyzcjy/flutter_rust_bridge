use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::lang::rust::RustLang;
use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::codec::structs::get_interest_types_for_codec;
use crate::codegen::generator::codec::structs::EncodeOrDecode;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, CodecMode};
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::entrypoint::CstWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::entrypoint::DcoWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::codec::pde::entrypoint::PdeWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::entrypoint::SseWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen_codec_structs;
use serde::Serialize;
use strum::IntoEnumIterator;

codegen_codec_structs!(Rust);

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
