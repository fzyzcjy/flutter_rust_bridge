use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::raw::HirRawPack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::library::commands::cargo_expand::run_cargo_expand;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    dumper: &Dumper,
) -> anyhow::Result<HirRawPack> {
    run_cargo_expand(&config.rust_crate_dir, dumper)
}
