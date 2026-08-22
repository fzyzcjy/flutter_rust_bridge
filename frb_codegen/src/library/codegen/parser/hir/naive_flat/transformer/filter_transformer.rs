use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItem;
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::hir::tree::transformer::pub_use_transformer::is_localized_definition;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

pub(crate) fn transform(
    mut pack: HirNaiveFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirNaiveFlatPack> {
    pack.items = (pack.items.drain(..))
        .filter(|item| is_interest(item, config))
        .collect_vec();
    Ok(pack)
}

fn is_interest(item: &HirNaiveFlatItem, config: &ParserHirInternalConfig) -> bool {
    (is_public_or_self_crate(item) || !is_localized_definition(&item.item))
        && !is_early_skip_namespace(&item.meta.namespace, config)
}

fn is_public_or_self_crate(item: &HirNaiveFlatItem) -> bool {
    // If it is third party crate, then we only scan the `pub` mods and items,
    // since for non-pub modes, it is impossible to use them even if we scanned them.
    is_self_crate(item) || item.meta.is_module_public
}

fn is_self_crate(item: &HirNaiveFlatItem) -> bool {
    item.meta.namespace.crate_name().is_self_crate()
}

fn is_early_skip_namespace(namespace: &Namespace, config: &ParserHirInternalConfig) -> bool {
    (config.rust_input_namespace_pack.rust_output_path_namespace).is_prefix_of(namespace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
    use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
    use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
    use std::path::PathBuf;
    use syn::parse_quote;

    fn config() -> ParserHirInternalConfig {
        ParserHirInternalConfig {
            rust_input_namespace_pack: RustInputNamespacePack {
                rust_input_namespace_prefixes: vec![],
                rust_output_path_namespace: Namespace::new_raw("crate::generated".to_owned()),
            },
            rust_crate_dir: PathBuf::new(),
            third_party_crate_names: vec![],
            rust_features: None,
            parse_const: false,
        }
    }

    fn item(namespace: &str, public_module: bool, item: syn::Item) -> HirNaiveFlatItem {
        HirNaiveFlatItem {
            meta: HirNaiveFlatItemMeta {
                namespace: Namespace::new_raw(namespace.to_owned()),
                sources: vec![HirGenerationSource::Normal],
                is_module_public: public_module,
            },
            item,
        }
    }

    /// Keeps self-crate and non-local items but skips generated and private external definitions.
    #[test]
    fn filters_items_by_origin_visibility_and_generated_namespace() -> anyhow::Result<()> {
        let filtered = transform(
            HirNaiveFlatPack {
                items: vec![
                    item(
                        "crate::api",
                        false,
                        parse_quote!(
                            struct SelfPrivate;
                        ),
                    ),
                    item(
                        "dependency::api",
                        true,
                        parse_quote!(
                            pub struct External;
                        ),
                    ),
                    item(
                        "dependency::api",
                        false,
                        parse_quote!(
                            struct PrivateExternal;
                        ),
                    ),
                    item("dependency::api", false, parse_quote!(impl External {})),
                    item(
                        "crate::generated",
                        true,
                        parse_quote!(
                            pub struct Generated;
                        ),
                    ),
                ],
            },
            &config(),
        )?;

        assert_eq!(filtered.items.len(), 3);
        assert!(matches!(filtered.items[0].item, syn::Item::Struct(_)));
        assert!(matches!(filtered.items[1].item, syn::Item::Struct(_)));
        assert!(matches!(filtered.items[2].item, syn::Item::Impl(_)));
        Ok(())
    }
}
