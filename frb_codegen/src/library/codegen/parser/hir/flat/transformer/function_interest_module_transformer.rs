use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use itertools::Itertools;
use crate::codegen::misc::THIRD_PARTY_NAMESPACE;

pub(crate) fn transform(
    mut pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    pack.functions = (pack.functions.drain(..))
        .filter(|f| {
            (config.rust_input_namespace_pack).is_interest(&f.namespace)
                || THIRD_PARTY_NAMESPACE.is_prefix_of(&f.namespace)
        })
        .collect_vec();
    Ok(pack)
}
