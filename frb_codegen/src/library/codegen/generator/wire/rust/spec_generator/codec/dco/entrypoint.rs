use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypointTrait, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;

pub(crate) struct DcoWireRustCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireRustGeneratorContext<'_>, WireRustCodecOutputSpec>
    for DcoWireRustCodecEntrypoint
{
    fn generate(
        &self,
        context: WireRustGeneratorContext,
        types: &[IrType],
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
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn generate_func_params(
        &self,
        _func: &IrFunc,
        _context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        unreachable!()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn generate_func_call_decode(
        &self,
        _func: &IrFunc,
        _context: WireRustGeneratorContext,
    ) -> String {
        unreachable!()
    }
}
