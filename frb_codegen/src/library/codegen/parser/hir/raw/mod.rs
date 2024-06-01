use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::raw::HirRawPack;
use crate::codegen::ir::mir::namespace::Namespace;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::library::commands::cargo_expand::run_cargo_expand;
use crate::library::commands::cargo_metadata::execute_cargo_metadata;
use cargo_metadata::Metadata;
use itertools::concat;
use std::path::PathBuf;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    dumper: &Dumper,
) -> anyhow::Result<HirRawPack> {
    let metadata = execute_cargo_metadata(&config.rust_crate_dir.join("Cargo.toml"))?;

    let crates = concat([
        vec![Namespace::SELF_CRATE.to_owned()],
        config.third_party_crates.clone(),
    ])
    .iter()
    .map(|crate_name| {
        let interest_crate_dir = compute_crate_dir(&crate_name, &metadata)?;
        Ok((
            crate_name.to_owned(),
            run_cargo_expand(&interest_crate_dir, dumper)?,
        ))
    })
    .collect::<anyhow::Result<Vec<_>>>()?
    .into_iter()
    .collect();
    Ok(HirRawPack { crates })
}

fn compute_crate_dir(crate_name: &str, metadata: &Metadata) -> anyhow::Result<PathBuf> {
    TODO
}
