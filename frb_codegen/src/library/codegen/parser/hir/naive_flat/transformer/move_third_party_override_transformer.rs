use crate::codegen::ir::hir::misc::syn_item_with_meta::SynItemWithMeta;
use crate::codegen::ir::hir::misc::visibility::HirVisibility;
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::codegen::misc::SELF_CRATE_THIRD_PARTY_NAMESPACE;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

pub(crate) fn transform(mut pack: HirNaiveFlatPack) -> anyhow::Result<HirNaiveFlatPack> {
    pack.items = (pack.items.drain(..))
        .map(|item| {
            if SELF_CRATE_THIRD_PARTY_NAMESPACE.is_prefix_of(&item.meta.namespace) {
                SynItemWithMeta {
                    meta: HirTreeModuleMeta {
                        namespace: compute_moved_namespace(&item.meta.namespace),
                        // hacky fake data
                        parent_vis: vec![],
                        vis: HirVisibility::Public,
                    },
                    item: item.item,
                }
            } else {
                item
            }
        })
        .collect_vec();
    Ok(pack)
}

fn compute_moved_namespace(original_namespace: &Namespace) -> Namespace {
    TODO
}
