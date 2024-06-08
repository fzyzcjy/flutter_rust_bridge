use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) mod generator;

pub(crate) fn transform(
    mut pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    TODO
}
