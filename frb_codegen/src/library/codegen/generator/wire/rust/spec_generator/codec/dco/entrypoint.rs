use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypointTrait, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::ty::MirType;

pub(crate) struct DcoWireRustCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireRustGeneratorContext<'_>, WireRustCodecOutputSpec>
    for DcoWireRustCodecEntrypoint
{
    fn generate(
        &self,
        context: WireRustGeneratorContext,
        types: &[MirType],
        mode: EncodeOrDecode,
    ) -> Option<WireRustCodecOutputSpec> {
        match mode {
            EncodeOrDecode::Encode => Some(super::encoder::generate(
                context.as_wire_rust_codec_dco_context(),
                types,
            )),
            EncodeOrDecode::Decode => None,
        }
    }
}

impl WireRustCodecEntrypointTrait<'_> for DcoWireRustCodecEntrypoint {
    // frb-coverage:ignore-start
    fn generate_func_params(
        &self,
        _func: &MirFunc,
        _context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        unreachable!()
    }

    fn generate_func_call_decode(
        &self,
        _func: &MirFunc,
        _context: WireRustGeneratorContext,
    ) -> String {
        unreachable!()
    }
    // frb-coverage:ignore-end
}
