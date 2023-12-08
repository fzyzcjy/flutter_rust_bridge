use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, CodecMode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::entrypoint::CstWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::WireDartCodecDcoGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::entrypoint::DcoWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::entrypoint::SseWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use crate::codegen_codec_structs;
use enum_dispatch::enum_dispatch;
use serde::Serialize;
use std::ops::Deref;

codegen_codec_structs!(WireDartCodec, WireDartOutputCode);

pub(crate) trait WireDartCodecEntrypointTrait<'a>:
    BaseCodecEntrypointTrait<WireDartGeneratorContext<'a>, WireDartCodecOutputSpec>
{
    fn generate_dart2rust_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String>;

    fn generate_dart2rust_func_wire_param_list(
        &self,
        func: &IrFunc,
        num_prepare_args: usize,
    ) -> Vec<String>;
}
