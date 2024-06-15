mod exporter;
pub(crate) mod extra_code_injector;
pub(crate) mod parser;
pub(crate) mod transformer;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::naive_flat::pack::HirNaiveFlatPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_naive_flat: HirNaiveFlatPack,
    dumper: &Dumper,
) -> anyhow::Result<HirFlatPack> {
    let pack = parser::pack::parse_pack(config, hir_naive_flat)?;
    dumper.dump("1_parse_pack.json", &pack)?;

    let pack = transformer::filter_transformer::transform(pack, config)?;
    dumper.dump("2_filter_transformer.json", &pack)?;

    let pack = transformer::remove_not_defined_trait_transformer::transform(pack)?;
    dumper.dump("3_remove_not_defined_trait_transformer.json", &pack)?;

    let pack = transformer::copy_trait_def_to_impl_transformer::transform(pack)?;
    dumper.dump("4_copy_trait_def_to_impl_transformer.json", &pack)?;

    let pack = transformer::function_frb_override_transformer::transform(pack)?;
    dumper.dump("5_function_frb_override_transformer.json", &pack)?;

    let pack = transformer::merge_duplicate_transformer::transform(pack)?;
    dumper.dump("6_merge_duplicate_transformer.json", &pack)?;

    let pack = transformer::resolve_type_alias_transformer::transform(pack)?;
    dumper.dump("7_resolve_type_alias_transformer.json", &pack)?;

    let pack = transformer::sort_transformer::transform(pack)?;
    dumper.dump("8_sort_transformer.json", &pack)?;

    Ok(pack)
}
