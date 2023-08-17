//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge
#![allow(clippy::vec_init_then_push)]

pub use crate::commands::ensure_tools_available;
pub use crate::config::opts::Opts;
pub use crate::config::opts_parser::config_parse;
pub use crate::config::raw_opts::RawOpts;
pub use crate::logs::init_logger;
pub use crate::utils::misc::get_symbols_if_no_duplicates;

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
use crate::utils::misc::BlockIndex;
pub(crate) use error::Result;
use log::info;

/// When the API is only defined in 1 rust file(block), take this one for generation, where `config`
/// is the instance containing all information to the API file(block), and `all_symbols` contains
/// all unique APIs defined in the file mentioned above.
/// If APIs are defined in more than 1 rust file(block), use `frb_codegen_multi` instead.
pub fn frb_codegen(config: &Opts, all_symbols: &[String]) -> anyhow::Result<()> {
    frb_codegen_multi(&[config.clone()], 0, all_symbols)
}

/// This function is used only for cases with multi-blocks when there are
/// more than one API block.
/// In addition, because the current block to deal with needs information
/// from all other blocks, `all_configs` is used here,
/// with `index` referring to the place of the current block to deal with.
/// For details on how to take advantage of multi-blocks, please refers to
/// this article: https://cjycode.com/flutter_rust_bridge/feature/multiple_files.html
pub fn frb_codegen_multi(
    all_configs: &[Opts],
    index: usize,
    all_symbols: &[String],
) -> anyhow::Result<()> {
    info!("Phase: Validate config(s)");
    for (i, config) in all_configs.iter().enumerate() {
        assert_eq!(
            BlockIndex(i),
            config.block_index,
            "order index({}) != block_index({})",
            i,
            config.block_index
        );
    }

    let config = &all_configs[index];
    info!("Picked config: {:?}", config);

    info!("Phase: Parse source code to AST, then to IR");
    let raw_ir_file = config.get_ir_file()?;

    info!("Phase: Transform IR");
    let ir_file = transformer::transform(raw_ir_file);

    info!("Phase: Generate Rust code");
    let generated_rust = generate_rust_code(config, &ir_file)?;

    info!("Phase: Generate Dart code");
    generate_dart_code(config, all_configs, &ir_file, generated_rust, all_symbols)?;

    info!("Success!");
    Ok(())
}
