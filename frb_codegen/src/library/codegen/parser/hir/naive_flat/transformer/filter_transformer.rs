use crate::codegen::ir::hir::naive_flat::item::{HirNaiveFlatItem, HirNaiveFlatItemVis};
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
    match item.meta.vis {
        HirNaiveFlatItemVis::Public => true,
        HirNaiveFlatItemVis::Private => false,
        // Impossible for third party crate to use such non-public items
        HirNaiveFlatItemVis::Others => is_self_crate(item),
    }
}

fn is_self_crate(item: &HirNaiveFlatItem) -> bool {
    item.meta.namespace.crate_name().is_self_crate()
}

fn is_early_skip_namespace(namespace: &Namespace, config: &ParserHirInternalConfig) -> bool {
    (config.rust_input_namespace_pack.rust_output_path_namespace).is_prefix_of(namespace)
}
