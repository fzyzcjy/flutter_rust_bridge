use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::ConfigDumpContent::Mir;

pub(crate) mod internal_config;
pub(crate) mod parser;
pub(crate) mod sanity_checker;
pub(crate) mod transformer;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    hir_flat: &HirFlatPack,
    dumper: &Dumper,
) -> anyhow::Result<MirPack> {
    let pack = parser::parse(config, hir_flat)?;
    dumper.dump("1_parse_pack", &pack)?;

    let pack = transformer::filter_trait_impl_transformer::transform(pack)?;
    dumper.dump("2_filter_trait_impl_transformer", &pack)?;

    // let pack = transformer::dyn_trait_inner_transformer::transform(pack)?;
    // dump(dumper, "3_dyn_trait_inner_transformer", &pack)?;

    Ok(pack)
}
