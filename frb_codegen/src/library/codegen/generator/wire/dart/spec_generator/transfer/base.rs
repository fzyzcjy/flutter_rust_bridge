use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::entrypoint::CstWireDartTransferEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::entrypoint::DcoWireDartTransferEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::transfer::sse::entrypoint::SseWireDartTransferEntrypoint;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(WireDartTransferEntrypointTrait)]
pub(crate) enum WireDartTransferEntrypoint {
    Cst(CstWireDartTransferEntrypoint),
    Dco(DcoWireDartTransferEntrypoint),
    Sse(SseWireDartTransferEntrypoint),
}

#[enum_dispatch]
pub(crate) trait WireDartTransferEntrypointTrait {}
