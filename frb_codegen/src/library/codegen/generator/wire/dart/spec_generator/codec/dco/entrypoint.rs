use crate::codegen::generator::codec::structs::BaseCodecEntrypointTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::WireDartCodecDcoGeneratorContext;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;

pub(crate) struct DcoWireDartCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for DcoWireDartCodecEntrypoint
{
    fn generate_encode(
        &self,
        _context: WireDartGeneratorContext,
        _types: &[IrType],
    ) -> Option<WireDartCodecOutputSpec> {
        None
    }

    fn generate_decode(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
    ) -> Option<WireDartCodecOutputSpec> {
        Some(super::decoder::generate(
            context.as_wire_dart_codec_dco_context(),
            types,
        ))
    }
}

impl WireDartCodecEntrypointTrait<'_> for DcoWireDartCodecEntrypoint {
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
            DcoCodec(
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
