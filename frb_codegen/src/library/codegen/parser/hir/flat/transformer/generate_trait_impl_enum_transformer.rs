use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::mir::parser::tentative_parse_trait_impls;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let trait_impls = tentative_parse_trait_impls(&pack);

    Ok(pack)
}
