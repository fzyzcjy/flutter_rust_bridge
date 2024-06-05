pub(crate) mod parser;
pub(crate) mod transformer;

use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_tree: HirTreePack,
) -> anyhow::Result<HirFlatPack> {
    let pack = parser::pack::parse_pack(config, hir_tree)?;
    let pack = transformer::trait_impl_transformer::transform(pack)?;
    let pack = transformer::third_party_override_transformer::transform(pack)?;
    Ok(pack)
}
