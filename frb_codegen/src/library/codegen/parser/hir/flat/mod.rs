pub(crate) mod parser;
pub(crate) mod transformer;

use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_tree: HirTreePack,
) -> anyhow::Result<HirFlatPack> {
    let pack = parser::pack::parse_pack(config, hir_tree)?;
    let pack = transformer::copy_trait_def_to_impl_transformer::transform(pack)?;
    let pack = transformer::merge_duplicate_transformer::transform(pack)?;
    let pack = transformer::resolve_type_alias_transformer::transform(pack)?;
    let pack = transformer::visibility_transformer::transform(pack)?;
    Ok(pack)
}
