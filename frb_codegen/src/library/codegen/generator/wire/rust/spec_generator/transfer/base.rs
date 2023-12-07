use crate::codegen::generator::misc::transfer::TransferMode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::entrypoint::CstWireRustTransferEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::entrypoint::DcoWireRustTransferEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::transfer::sse::entrypoint::SseWireRustTransferEntrypoint;
use crate::codegen::ir::func::IrFunc;
use crate::codegen_transfer_structs;
use enum_dispatch::enum_dispatch;

codegen_transfer_structs!(WireRustTransferEntrypoint);

#[enum_dispatch]
pub(crate) trait WireRustTransferEntrypointTrait {}
