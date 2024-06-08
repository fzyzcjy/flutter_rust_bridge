pub(crate) mod trait_impl_enum;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;

pub(crate) fn execute(
    pack: HirFlatPack,
    config_mir: &ParserMirInternalConfig,
    config_hir: &ParserHirInternalConfig,
    dumper: &Dumper,
) -> anyhow::Result<HirFlatPack> {
    let dumper_tentative_mir = dumper.with_add_name_prefix("tentative_mir/");
    let tentative_mir_pack = mir::parse(config_mir, &pack, &dumper_tentative_mir)?;

    let pack = trait_impl_enum::transform(pack, &tentative_mir_pack, config_hir)?;

    Ok(pack)
}
