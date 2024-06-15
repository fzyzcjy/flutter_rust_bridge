use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) mod parser;
pub(crate) mod transformer;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_tree: HirTreePack,
    dumper: &Dumper,
) -> anyhow::Result<HirNaiveFlatPack> {
    let pack = parser::parse(hir_tree)?;
    dumper.dump("1_parse_pack.json", &pack)?;

    let pack = transformer::move_third_party_override_transformer::transform(pack)?;
    dumper.dump("2_move_third_party_override_transformer.json", &pack)?;

    let pack = transformer::filter_transformer::transform(pack, config)?;
    dumper.dump("3_filter_transformer.json", &pack)?;

    Ok(pack)
}
