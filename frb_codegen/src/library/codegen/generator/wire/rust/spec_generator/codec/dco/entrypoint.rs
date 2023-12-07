use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::WireRustCodecEntrypointTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::ir::func::IrFunc;

pub(crate) struct DcoWireRustCodecEntrypoint {}

impl WireRustCodecEntrypointTrait for DcoWireRustCodecEntrypoint {
    fn generate_func_params(
        &self,
        _func: &IrFunc,
        _context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        unreachable!()
    }

    fn generate_func_call_decode(
        &self,
        _func: &IrFunc,
        _context: WireRustGeneratorContext,
    ) -> String {
        unreachable!()
    }
}
