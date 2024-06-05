use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_tree: HirTreePack,
) -> anyhow::Result<HirFlatPack> {
    Ok(TODO)
}
