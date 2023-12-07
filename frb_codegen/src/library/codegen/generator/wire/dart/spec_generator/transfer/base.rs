use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::entrypoint::CstWireDartTransferEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::entrypoint::DcoWireDartTransferEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::sse::entrypoint::SseWireDartTransferEntrypoint;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::misc::transfer::TransferMode;
use enum_dispatch::enum_dispatch;

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
}
