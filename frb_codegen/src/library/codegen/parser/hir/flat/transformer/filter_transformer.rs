use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::misc::SELF_CRATE_THIRD_PARTY_NAMESPACE;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use syn::Visibility;

pub(crate) fn transform(
    mut pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    filter_function(&mut pack, config);
    Ok(pack)
}

fn filter_function(pack: &mut HirFlatPack, config: &ParserHirInternalConfig) {
    pack.functions = (pack.functions.drain(..))
        .filter(|f| {
            is_interest_module(&f.namespace, config)
                && (f.namespace.crate_name().is_self_crate()
                    || f.source == HirGenerationSource::MoveFromCrateThirdPartyFolder
                    || is_function_public(f))
        })
        .collect_vec();
}

fn is_interest_module(namespace: &Namespace, config: &ParserHirInternalConfig) -> bool {
    (config.rust_input_namespace_pack).is_interest(namespace)
        || SELF_CRATE_THIRD_PARTY_NAMESPACE.is_prefix_of(namespace)
}

fn is_function_public(f: &HirFlatFunction) -> bool {
    f.item_fn
        .vis()
        .map(|v| matches!(v, Visibility::Public(_)))
        .unwrap_or(true)
}
