use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecEntrypointTrait;
use crate::codegen::ir::func::IrFunc;

pub(crate) struct DcoWireDartCodecEntrypoint {}

impl WireDartCodecEntrypointTrait for DcoWireDartCodecEntrypoint {
    fn generate_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String> {
        unreachable!()
    }

    fn generate_func_wire_param_list(&self, func: &IrFunc, num_prepare_args: usize) -> Vec<String> {
        unreachable!()
    }
}
