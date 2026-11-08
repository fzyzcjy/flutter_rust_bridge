use crate::codegen::dumper::Dumper;
use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::sanity_checker::safe_ident_collision_checker::check_safe_ident_collisions;

pub(crate) mod internal_config;
pub(crate) mod parser;
pub(crate) mod sanity_checker;
pub(crate) mod transformer;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    ir_pack: &IrEarlyGeneratorPack,
    dumper: &Dumper,
    parse_mode: ParseMode,
) -> anyhow::Result<MirPack> {
    let pack = parser::parse(config, ir_pack, parse_mode)?;
    dumper.dump("1_parse_pack.json", &pack)?;
    if parse_mode == ParseMode::Normal {
        check_safe_ident_collisions(&pack)?;
    }

    let pack = transformer::filter_trait_impl_transformer::transform(pack)?;
    dumper.dump("2_filter_trait_impl_transformer.json", &pack)?;

    // let pack = transformer::dyn_trait_inner_transformer::transform(pack)?;
    // dump(dumper, "3_dyn_trait_inner_transformer", &pack)?;

    Ok(pack)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum ParseMode {
    Early,
    Normal,
}
