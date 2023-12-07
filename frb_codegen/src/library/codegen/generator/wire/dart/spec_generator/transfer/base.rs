use crate::codegen::generator::misc::transfer::CodecMode;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::entrypoint::CstWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::entrypoint::DcoWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::sse::entrypoint::SseWireDartCodecEntrypoint;
use crate::codegen::ir::func::IrFunc;
use crate::codegen_transfer_structs;
use enum_dispatch::enum_dispatch;

codegen_transfer_structs!(WireDartCodecEntrypoint);

#[enum_dispatch]
pub(crate) trait WireDartCodecEntrypointTrait {
    fn generate_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String>;

    fn generate_func_wire_param_list(&self, func: &IrFunc, num_prepare_args: usize) -> Vec<String>;
}
