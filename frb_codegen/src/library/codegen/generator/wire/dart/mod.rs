use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};

mod api2wire;
pub(crate) mod base;
pub(crate) mod misc;
mod output_code;
mod wire2api;

pub(crate) fn generate(
    ir_pack: &IrPack,
    context: WireDartGeneratorContext,
) -> Acc<WireDartOutputCode> {
    let cache = IrPackComputedCache::compute(ir_pack);
    let mut ans = Acc::default();
    ans += misc::generate(context, &cache);
    ans += wire2api::generate(context, &cache);
    ans += api2wire::generate(context, &cache);
    ans.map(|v, _| v.into_iter().collect())
}
