use crate::codegen::generator::misc::transfer::TransferMode;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::entrypoint::CstWireDartTransferEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::entrypoint::DcoWireDartTransferEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::sse::entrypoint::SseWireDartTransferEntrypoint;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::func::IrFunc;
use enum_dispatch::enum_dispatch;
use std::{format, vec};

#[enum_dispatch(WireDartTransferEntrypointTrait)]
pub(crate) enum WireDartTransferEntrypoint {
    Cst(CstWireDartTransferEntrypoint),
    Dco(DcoWireDartTransferEntrypoint),
    Sse(SseWireDartTransferEntrypoint),
}

impl WireDartTransferEntrypoint {
    pub(crate) fn new(mode: TransferMode) -> Self {
        match mode {
            TransferMode::Cst => Self::Cst(CstWireDartTransferEntrypoint {}),
            TransferMode::Dco => Self::Dco(DcoWireDartTransferEntrypoint {}),
            TransferMode::Sse => Self::Sse(SseWireDartTransferEntrypoint {}),
        }
    }
}

#[enum_dispatch]
pub(crate) trait WireDartTransferEntrypointTrait {
    fn generate_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String>;

    fn generate_func_wire_param_list(&self, func: &IrFunc, num_prepare_args: usize) -> Vec<String>;
}
