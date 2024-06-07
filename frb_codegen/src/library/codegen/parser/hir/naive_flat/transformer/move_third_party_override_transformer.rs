use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::naive_flat::item::{HirNaiveFlatItem, HirNaiveFlatItemMeta};
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::misc::SELF_CRATE_THIRD_PARTY_NAMESPACE;
use crate::utils::namespace::Namespace;
use itertools::{concat, Itertools};

pub(crate) fn transform(mut pack: HirNaiveFlatPack) -> anyhow::Result<HirNaiveFlatPack> {
    pack.items = (pack.items.drain(..))
        .map(|item| {
            if SELF_CRATE_THIRD_PARTY_NAMESPACE.is_prefix_of(&item.meta.namespace) {
                HirNaiveFlatItem {
                    meta: HirNaiveFlatItemMeta {
                        namespace: compute_moved_namespace(&item.meta.namespace),
                        sources: concat([
                            item.meta.sources.clone(),
                            vec![HirGenerationSource::MoveFromCrateThirdPartyFolder],
                        ]),
                        is_module_public: true,
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

fn compute_moved_namespace(original: &Namespace) -> Namespace {
    original.strip_prefix(&SELF_CRATE_THIRD_PARTY_NAMESPACE)
}
