use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::output_code::WireRustOutputCode;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

pub(crate) mod api2wire;
pub(crate) mod base;
pub(in crate::library::codegen::generator::wire::rust) mod extern_func;
mod internal_config;
pub(crate) mod misc;
mod output_code;
pub(crate) mod wire2api;

pub(crate) fn generate(
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let cache = IrPackComputedCache::compute(ir_pack);
    let mut ans = Acc::default();
    ans += misc::generate(ir_pack, context, &cache);
    ans += wire2api::generate(context, &cache);
    ans += api2wire::generate(context, &cache);
    ans.map(|v, _| v.into_iter().collect())
}
