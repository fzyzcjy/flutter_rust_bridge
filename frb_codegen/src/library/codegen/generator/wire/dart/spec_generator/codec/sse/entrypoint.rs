use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecEntrypointTrait;
use crate::codegen::ir::func::IrFunc;

pub(crate) struct SseWireDartCodecEntrypoint {}

impl WireDartCodecEntrypointTrait for SseWireDartCodecEntrypoint {
    fn generate_dart2rust_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String> {
        todo!()
    }

    fn generate_dart2rust_func_wire_param_list(
        &self,
        func: &IrFunc,
        num_prepare_args: usize,
    ) -> Vec<String> {
        todo!()
    }

    fn generate_rust2dart_codec_object(&self, func: &IrFunc) -> String {
        todo!()
    }
}
