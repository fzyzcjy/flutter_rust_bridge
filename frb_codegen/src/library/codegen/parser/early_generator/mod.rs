mod proxy_enum;
mod trait_impl_enum;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::hir::flat::extra_code_injector::inject_extra_code;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::mir;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;

pub(crate) fn execute(
    mut pack: HirFlatPack,
    config_mir: &ParserMirInternalConfig,
    dumper: &Dumper,
) -> anyhow::Result<HirFlatPack> {
    let dumper_tentative_mir = dumper.with_add_name_prefix("1_tentative_mir/");
    let tentative_mir_pack = mir::parse(config_mir, &pack, &dumper_tentative_mir)?;

    trait_impl_enum::generate(&mut pack, &tentative_mir_pack, config_mir)?;
    dumper.dump("2_trait_impl_enum.json", &pack)?;

    proxy_enum::generate(&mut pack, &tentative_mir_pack, config_mir)?;
    dumper.dump("3_proxy_enum.json", &pack)?;

    Ok(pack)
}

fn inject_extra_code_to_rust_output(
    pack: &mut HirFlatPack,
    extra_code: &str,
    config_mir: &ParserMirInternalConfig,
) -> anyhow::Result<()> {
    inject_extra_code(
        pack,
        &extra_code,
        &(config_mir.rust_input_namespace_pack).rust_output_path_namespace,
    )
}
