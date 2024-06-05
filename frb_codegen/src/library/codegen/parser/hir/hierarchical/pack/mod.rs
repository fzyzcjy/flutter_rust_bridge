use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::raw::pack::HirRawPack;
use crate::codegen::parser::hir::hierarchical::crates::parse_crate;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

mod third_party_override_transformer;
mod trait_impl_transformer;

pub(crate) fn parse_pack(
    config: &ParserHirInternalConfig,
    hir_raw: &HirRawPack,
) -> anyhow::Result<HirPack> {
    let pack = parse_raw(config, hir_raw)?;
    let pack = trait_impl_transformer::transform(pack)?;
    let pack = third_party_override_transformer::transform(pack)?;
    Ok(pack)
}

// moved
// fn parse_raw(config: &ParserHirInternalConfig, hir_raw: &HirRawPack) -> anyhow::Result<HirPack> {
// }
