use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::misc::section_header_comment;
use crate::codegen::generator::wire::rust::wire2api::extern_func::CodeWithExternFunc;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

pub(crate) mod api2wire;
pub(crate) mod base;
mod internal_config;
pub(crate) mod misc;
pub(crate) mod wire2api;

pub(crate) fn generate(
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
) -> Acc<CodeWithExternFunc> {
    let cache = IrPackComputedCache::new(ir_pack);
    let mut ans = Acc::default();
    ans += misc::generate(ir_pack, context, &cache);
    ans += wire2api::generate(context, &cache);
    ans += api2wire::generate(context, &cache);
    ans
}

/// Some information derivable from `IrPack`, but may be expensive to compute,
/// so we compute once and cache them.
struct IrPackComputedCache {
    distinct_input_types: Vec<IrType>,
    distinct_output_types: Vec<IrType>,
    input_and_output_types: Vec<IrType>,
}

impl IrPackComputedCache {
    pub fn new(ir_pack: &IrPack) -> Self {
        let distinct_input_types = ir_pack.distinct_types(true, false);
        let distinct_output_types = ir_pack.distinct_types(false, true);
        let input_and_output_types = distinct_input_types
            .iter()
            .cloned()
            .chain(distinct_output_types.iter().cloned())
            .collect_vec();
        Self {
            distinct_input_types,
            distinct_output_types,
            input_and_output_types,
        }
    }
}
