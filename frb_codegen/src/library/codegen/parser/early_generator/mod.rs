pub(crate) mod trait_impl_enum;

use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;

pub(crate) fn execute(
    pack: HirFlatPack,
    config_mir: &ParserMirInternalConfig,
    config_hir: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    let tentative_mir_pack = mir::parse(config_mir, &pack, todo!())?;

    let pack = trait_impl_enum::transform(pack, &tentative_mir_pack, config_hir)?;

    Ok(pack)
}
