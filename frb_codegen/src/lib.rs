//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge
#![allow(clippy::vec_init_then_push)]

pub use crate::commands::ensure_tools_available;
pub use crate::config::opts::Opts;
pub use crate::config::opts_parser::parse_configs_and_symbols;
pub use crate::config::raw_opts::RawOpts;
pub use crate::logs::init_logger;

pub mod config;
pub mod dump;
pub mod utils;

mod commands;
mod entrypoint;
mod error;
mod generator;
mod ir;
mod logs;
mod others;
mod parser;
mod target;
mod transformer;

use crate::entrypoint::dart::generate_dart_code;
use crate::entrypoint::rust::generate_rust_code;
use config::all_configs::AllConfigs;
pub(crate) use error::Result;
use log::info;

/// When the API is only defined in 1 rust file(block), take this one for generation, where `config`
/// is the instance containing all information to the API file(block), and `all_symbols` contains
/// all unique APIs defined in the file mentioned above.
/// If APIs are defined in more than 1 rust file(block), use `frb_codegen_multi` instead.
pub fn frb_codegen(config: &Opts, all_symbols: &[String]) -> anyhow::Result<()> {
    frb_codegen_multi(config, &AllConfigs::new(vec![config.clone()]), all_symbols)
}

/// This function is used only for cases with multi-blocks when there are
/// more than one API block.
/// In addition, because the current block to deal with needs information
/// from all other blocks, `all_configs` is used here,
/// with `index` referring to the place of the current block to deal with.
/// For details on how to take advantage of multi-blocks, please refers to
/// this article: https://cjycode.com/flutter_rust_bridge/feature/multiple_files.html
pub fn frb_codegen_multi(
    config: &Opts,
    all_configs: &AllConfigs,
    all_symbols: &[String],
) -> anyhow::Result<()> {
    let config_name = config.get_name();
    info!("Picked `{config_name}`");

    info!("Phase: Generate Rust code for `{config_name}`");
    let generated_rust = generate_rust_code(config, all_configs)?;

    info!("Phase: Generate Dart code for `{config_name}`");
    generate_dart_code(config, all_configs, generated_rust, all_symbols)?;

    info!("Success!");
    Ok(())
}
