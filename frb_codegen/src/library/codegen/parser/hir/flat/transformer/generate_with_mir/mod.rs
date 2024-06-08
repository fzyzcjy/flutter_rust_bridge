use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) mod generator;

pub(crate) fn transform(
    pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    let mir_pack = tentatively_run_mir(&pack)?;

    let pack = generator::trait_impl_enum::transform(pack, &mir_pack, config)?;

    Ok(pack)
}

fn tentatively_run_mir(pack: &HirFlatPack) -> anyhow::Result<MirPack> {
    TODO;
}
