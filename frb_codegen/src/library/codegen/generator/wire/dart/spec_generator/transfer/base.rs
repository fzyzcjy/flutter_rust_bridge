use crate::codegen::generator::misc::transfer::TransferMode;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::entrypoint::CstWireDartTransferEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::entrypoint::DcoWireDartTransferEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::sse::entrypoint::SseWireDartTransferEntrypoint;
use crate::codegen::ir::func::IrFunc;
use crate::codegen_transfer_structs;
use enum_dispatch::enum_dispatch;

codegen_transfer_structs!(WireDartTransferEntrypoint);

#[enum_dispatch]
pub(crate) trait WireDartTransferEntrypointTrait {
    fn generate_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String>;

    fn generate_func_wire_param_list(&self, func: &IrFunc, num_prepare_args: usize) -> Vec<String>;
}
