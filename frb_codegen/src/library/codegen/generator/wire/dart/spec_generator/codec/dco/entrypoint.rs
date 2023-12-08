use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
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
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
        mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        match mode {
            EncodeOrDecode::Encode => None,
            EncodeOrDecode::Decode => Some(super::decoder::generate(
                context.as_wire_dart_codec_dco_context(),
                types,
            )),
        }
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
}
