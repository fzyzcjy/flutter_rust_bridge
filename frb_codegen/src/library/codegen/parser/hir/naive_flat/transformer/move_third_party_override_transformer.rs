use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use itertools::Itertools;

pub(crate) fn transform(mut pack: HirNaiveFlatPack) -> anyhow::Result<HirNaiveFlatPack> {
    pack.items = (pack.items.drain(..))
        .map(|item| {
            TODO;
            TODO
        })
        .collect_vec();
    Ok(pack)
}
