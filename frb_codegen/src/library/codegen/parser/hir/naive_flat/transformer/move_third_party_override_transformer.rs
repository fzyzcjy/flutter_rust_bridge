use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::misc::SELF_CRATE_THIRD_PARTY_NAMESPACE;
use itertools::Itertools;

pub(crate) fn transform(mut pack: HirNaiveFlatPack) -> anyhow::Result<HirNaiveFlatPack> {
    pack.items = (pack.items.drain(..))
        .map(|item| {
            if SELF_CRATE_THIRD_PARTY_NAMESPACE.is_prefix_of(&item.meta.namespace) {
                TODO
            } else {
                item
            }
        })
        .collect_vec();
    Ok(pack)
}
