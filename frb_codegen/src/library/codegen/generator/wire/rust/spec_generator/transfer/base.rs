use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::transfer::TransferMode;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::entrypoint::CstWireRustTransferEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::transfer::dco::entrypoint::DcoWireRustTransferEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::transfer::sse::entrypoint::SseWireRustTransferEntrypoint;
use crate::codegen::ir::func::IrFunc;
use crate::codegen_transfer_structs;
use enum_dispatch::enum_dispatch;

codegen_transfer_structs!(WireRustTransferEntrypoint);

#[enum_dispatch]
pub(crate) trait WireRustTransferEntrypointTrait {
    fn generate_func_params(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>>;

    fn generate_func_call_decode(&self, func: &IrFunc, context: WireRustGeneratorContext)
        -> String;
}
