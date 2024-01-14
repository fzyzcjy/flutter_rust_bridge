use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;

pub(crate) struct PdeWireDartCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for PdeWireDartCodecEntrypoint
{
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
        mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        todo!()
    }
}

impl WireDartCodecEntrypointTrait<'_> for PdeWireDartCodecEntrypoint {
    fn generate_dart2rust_inner_func_stmt(&self, func: &IrFunc, wire_func_name: &str) -> String {
        todo!()
    }
}
