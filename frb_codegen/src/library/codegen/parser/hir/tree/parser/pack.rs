use crate::codegen::ir::hir::raw::pack::HirRawPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::parser::hir::tree::parser::crates::parse_crate;

pub(crate) fn parse_pack(
    config: &ParserHirInternalConfig,
    hir_raw: HirRawPack,
) -> anyhow::Result<HirTreePack> {
    let crates = (hir_raw.crates.into_iter())
        .map(|c| parse_crate(config, c.syn_file, &c.name))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .collect();
    Ok(HirTreePack { crates })
}
