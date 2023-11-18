use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::codegen::ir::pack::IrPack;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> Acc<Vec<String>> {
    let mut lines = Acc::<Vec<_>>::default();
    todo!();
    lines
}
