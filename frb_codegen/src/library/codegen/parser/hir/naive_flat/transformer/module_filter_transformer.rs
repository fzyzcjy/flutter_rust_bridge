use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItem;
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::hir::tree::transformer::pub_use_transformer::is_localized_definition;
use crate::utils::crate_name::CrateName;
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
    (is_public_mod_or_self_crate(item) || !is_localized_definition(&item.item))
        && !is_early_skip_namespace(&item.meta.namespace, config)
}

fn is_public_mod_or_self_crate(item: &HirNaiveFlatItem) -> bool {
    // If it is third party crate, then we only scan the `pub` mods,
    // since for non-pub modes, it is impossible to use them even if we scanned them.
    item.meta.namespace.path()[0] == CrateName::SELF_CRATE || item.meta.is_public
}

fn is_early_skip_namespace(namespace: &Namespace, config: &ParserHirInternalConfig) -> bool {
    (config.rust_input_namespace_pack)
        .early_skip_namespace_prefixes
        .iter()
        .any(|prefix| prefix.is_prefix_of(namespace))
}