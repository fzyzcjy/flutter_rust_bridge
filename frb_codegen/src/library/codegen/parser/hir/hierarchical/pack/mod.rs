use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::raw::pack::HirRawPack;
use crate::codegen::parser::hir::hierarchical::crates::parse_crate;
use crate::codegen::parser::hir::hierarchical::third_party_override_transformer;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;

pub(crate) fn parse_pack(
    config: &ParserHirInternalConfig,
    hir_raw: &HirRawPack,
) -> anyhow::Result<HirPack> {
    let pack = parse_raw(config, hir_raw)?;
    let pack = third_party_override_transformer::transform(pack)?;
    Ok(pack)
}

fn parse_raw(config: &ParserHirInternalConfig, hir_raw: &HirRawPack) -> anyhow::Result<HirPack> {
    let crates = hir_raw
        .crates
        .iter()
        .map(|c| parse_crate(config, &c.syn_file, &c.name))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .collect();
    Ok(HirPack { crates })
}
