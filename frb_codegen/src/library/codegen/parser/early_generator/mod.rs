mod proxy_enum;
mod sorter;
pub(crate) mod trait_impl_enum;
pub(crate) mod ui_related;
pub(crate) mod utils;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::mir;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;

pub(crate) fn execute(
    hir_flat_pack: HirFlatPack,
    config_mir: &ParserMirInternalConfig,
    dumper: &Dumper,
) -> anyhow::Result<IrEarlyGeneratorPack> {
    let mut pack = IrEarlyGeneratorPack {
        hir_flat_pack,
        ..Default::default()
    };

    let dumper_tentative_mir = dumper.with_add_name_prefix("1_tentative_mir/");
    let tentative_mir_pack = mir::parse(
        config_mir,
        &pack,
        &dumper_tentative_mir,
        mir::ParseMode::Early,
    )?;

    trait_impl_enum::generate(&mut pack, &tentative_mir_pack, config_mir)?;
    dumper.dump("2_trait_impl_enum.json", &pack)?;

    proxy_enum::generate(&mut pack, &tentative_mir_pack, config_mir)?;
    dumper.dump("3_proxy_enum.json", &pack)?;

    ui_related::generate(&mut pack, config_mir)?;
    dumper.dump("4_ui_related.json", &pack)?;

    sorter::generate(&mut pack);
    dumper.dump("5_sorter.json", &pack)?;

    Ok(pack)
}
