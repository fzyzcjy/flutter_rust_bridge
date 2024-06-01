use crate::codegen::dumper::Dumper;
use crate::codegen::parser::reader::internal_config::ParserReaderInternalConfig;
use crate::library::commands::cargo_expand::run_cargo_expand;

pub(crate) mod internal_config;

pub(crate) fn parse(
    config: &ParserReaderInternalConfig,
    dumper: &Dumper,
) -> anyhow::Result<syn::File> {
    run_cargo_expand(&config.rust_crate_dir, dumper)
}
