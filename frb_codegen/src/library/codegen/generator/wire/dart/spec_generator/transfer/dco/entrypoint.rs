use crate::codegen::generator::wire::dart::spec_generator::transfer::base::WireDartTransferEntrypointTrait;
use crate::codegen::ir::func::IrFunc;

pub(crate) struct DcoWireDartTransferEntrypoint {}

impl WireDartTransferEntrypointTrait for DcoWireDartTransferEntrypoint {
    fn generate_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String> {
        unreachable!()
    }
}
