use crate::codegen::ir::hir::naive_flat::item::{HirNaiveFlatItem, HirNaiveFlatItemMeta};
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::utils::crate_name::CrateName;
use itertools::Itertools;

pub(crate) fn transform(mut pack: HirNaiveFlatPack) -> anyhow::Result<HirNaiveFlatPack> {
    pack.items = (pack.items.drain(..))
        .filter(|item| is_interest_mod(item))
        .collect_vec();
    Ok(pack)
}

fn is_interest_mod(item: &HirNaiveFlatItem) -> bool {
    // If it is third party crate, then we only scan the `pub` mods,
    // since for non-pub modes, it is impossible to use them even if we scanned them.
    item.meta.namespace.path()[0] == CrateName::SELF_CRATE || item.meta.is_public
}
