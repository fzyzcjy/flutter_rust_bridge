use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::get_interest_types_for_codec;
use crate::codegen::generator::codec::structs::EncodeOrDecode;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, CodecMode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::entrypoint::CstWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::entrypoint::DcoWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::pde::entrypoint::PdeWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::entrypoint::SseWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::codegen_codec_structs;
use serde::Serialize;
use strum::IntoEnumIterator;

codegen_codec_structs!(Dart);

pub(crate) trait WireDartCodecEntrypointTrait<'a>:
    BaseCodecEntrypointTrait<WireDartGeneratorContext<'a>, WireDartCodecOutputSpec>
{
    fn generate_dart2rust_inner_func_stmt(&self, func: &MirFunc, wire_func_name: &str) -> String;
}
