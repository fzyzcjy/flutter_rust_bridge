use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecEntrypointTrait;
use crate::codegen::ir::func::IrFunc;
use crate::library::codegen::ir::ty::IrTypeTrait;

pub(crate) struct DcoWireDartCodecEntrypoint {}

impl WireDartCodecEntrypointTrait for DcoWireDartCodecEntrypoint {
    fn generate_dart2rust_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String> {
        unreachable!()
    }

    fn generate_dart2rust_func_wire_param_list(
        &self,
        func: &IrFunc,
        num_prepare_args: usize,
    ) -> Vec<String> {
        unreachable!()
    }

    fn generate_rust2dart_codec_object(&self, func: &IrFunc) -> String {
        let parse_success_data = generate_parse_success_data(func);
        let parse_error_data = generate_parse_error_data(func);

        format!(
            "
            const DcoCodec(
              parseSuccessData: {parse_success_data},
              parseErrorData: {parse_error_data},
            )
            "
        )
    }
}

fn generate_parse_success_data(func: &IrFunc) -> String {
    format!("_dco_decode_{}", func.output.safe_ident())
}

fn generate_parse_error_data(func: &IrFunc) -> String {
    if let Some(error_output) = &func.error_output {
        format!("_dco_decode_{}", error_output.safe_ident())
    } else {
        "null".to_string()
    }
}
