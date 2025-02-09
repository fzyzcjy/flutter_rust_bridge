use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::misc::SELF_CRATE_THIRD_PARTY_NAMESPACE;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

pub(crate) fn transform(
    mut pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    filter_function(&mut pack, config);
    filter_constant(&mut pack, config);
    Ok(pack)
}

fn filter_function(pack: &mut HirFlatPack, config: &ParserHirInternalConfig) {
    pack.functions = (pack.functions.drain(..))
        .filter(|f| should_keep(&f.namespace, f.is_public().unwrap_or(true), config))
        .collect_vec();
}

fn filter_constant(pack: &mut HirFlatPack, config: &ParserHirInternalConfig) {
    pack.constants = (pack.constants.drain(..))
        .filter(|f| should_keep(&f.namespace, TODO, config))
        .collect_vec();
}

fn should_keep(namespace: &Namespace, is_public: bool, config: &ParserHirInternalConfig) -> bool {
    is_interest_module(&namespace, config) && (namespace.crate_name().is_self_crate() || is_public)
}

fn is_interest_module(namespace: &Namespace, config: &ParserHirInternalConfig) -> bool {
    (config.rust_input_namespace_pack).is_interest(namespace)
        || SELF_CRATE_THIRD_PARTY_NAMESPACE.is_prefix_of(namespace)
}
