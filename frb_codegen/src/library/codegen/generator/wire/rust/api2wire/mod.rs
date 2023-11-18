use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::WireRustGeneratorContext;
use crate::codegen::ir::pack::IrPack;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(ir_pack: &IrPack, context: WireRustGeneratorContext) -> Acc<Vec<String>> {
    todo!()
}
