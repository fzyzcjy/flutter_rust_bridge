use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::raw::HirRawPack;
use crate::codegen::ir::mir::namespace::Namespace;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::library::commands::cargo_expand::run_cargo_expand;
use itertools::concat;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    dumper: &Dumper,
) -> anyhow::Result<HirRawPack> {
    let crates = concat([
        vec![Namespace::SELF_CRATE.to_owned()],
        config.third_party_crates.clone(),
    ])
    .iter()
    .map(|crate_name| Ok((crate_name.to_owned(), run_cargo_expand(TODO, dumper)?)))
    .collect::<anyhow::Result<Vec<_>>>()?
    .into_iter()
    .collect();
    Ok(HirRawPack { crates })
}
