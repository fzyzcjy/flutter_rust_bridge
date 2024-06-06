use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::misc::SELF_CRATE_THIRD_PARTY_NAMESPACE;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use itertools::Itertools;

pub(crate) fn transform(
    mut pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    filter_function_interest_module(&mut pack, config);
    Ok(pack)
}

fn filter_function_interest_module(pack: &mut HirFlatPack, config: &ParserHirInternalConfig) {
    pack.functions = (pack.functions.drain(..))
        .filter(|f| {
            (config.rust_input_namespace_pack).is_interest(&f.namespace)
                || SELF_CRATE_THIRD_PARTY_NAMESPACE.is_prefix_of(&f.namespace)
        })
        .collect_vec();
}
