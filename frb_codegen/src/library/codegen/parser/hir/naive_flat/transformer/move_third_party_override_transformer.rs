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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
    use syn::parse_quote;

    /// Moves only self-crate third-party items and records their synthetic source.
    #[test]
    fn moves_third_party_items_to_their_original_namespace() -> anyhow::Result<()> {
        let transformed = transform(HirNaiveFlatPack {
            items: vec![
                HirNaiveFlatItem {
                    meta: HirNaiveFlatItemMeta {
                        namespace: Namespace::new_raw(
                            "crate::third_party::dependency::api".to_owned(),
                        ),
                        sources: vec![HirGenerationSource::Normal],
                        is_module_public: false,
                    },
                    item: parse_quote!(
                        struct Moved;
                    ),
                },
                HirNaiveFlatItem {
                    meta: HirNaiveFlatItemMeta {
                        namespace: Namespace::new_raw("crate::api".to_owned()),
                        sources: vec![HirGenerationSource::Normal],
                        is_module_public: false,
                    },
                    item: parse_quote!(
                        struct Kept;
                    ),
                },
            ],
        })?;

        assert_eq!(
            transformed.items[0].meta.namespace.to_string(),
            "dependency::api"
        );
        assert!(transformed.items[0].meta.is_module_public);
        assert_eq!(transformed.items[0].meta.sources.len(), 2);
        assert_eq!(
            transformed.items[1].meta.namespace.to_string(),
            "crate::api"
        );
        assert!(!transformed.items[1].meta.is_module_public);
        Ok(())
    }
}
