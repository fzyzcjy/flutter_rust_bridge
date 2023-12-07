use crate::codegen::generator::wire::dart::spec_generator::transfer::base::WireDartCodecEntrypointTrait;
use crate::codegen::ir::func::IrFunc;

pub(crate) struct SseWireDartCodecEntrypoint {}

impl WireDartCodecEntrypointTrait for SseWireDartCodecEntrypoint {
    fn generate_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String> {
        todo!()
    }

    fn generate_func_wire_param_list(&self, func: &IrFunc, num_prepare_args: usize) -> Vec<String> {
        todo!()
    }
}
