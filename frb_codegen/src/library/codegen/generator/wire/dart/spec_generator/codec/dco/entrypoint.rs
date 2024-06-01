use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::ty::MirType;

pub(crate) struct DcoWireDartCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for DcoWireDartCodecEntrypoint
{
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[MirType],
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
    // frb-coverage:ignore-start
    fn generate_dart2rust_inner_func_stmt(&self, _func: &MirFunc, _wire_func_name: &str) -> String {
        unreachable!()
    }
    // frb-coverage:ignore-end
}
