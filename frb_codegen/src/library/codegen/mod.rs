//! Code generator for `flutter_rust_bridge`

pub(crate) mod config;
mod generator;
pub(crate) mod ir;
pub(crate) mod parser;

pub use config::config::{Config, ConfigDump};
pub use config::config_parser::*;

use crate::codegen::config::internal_config::InternalConfig;
use log::debug;

/// Execute the main code generator
pub fn generate(config: Config) -> anyhow::Result<()> {
    debug!("config={config:?}");
    let internal_config = InternalConfig::parse(config)?;
    debug!("internal_config={internal_config:?}");

    let ir_pack = parser::parse(&internal_config.parser)?;

    // TODO seems not this ideal. we need to call various external tools that directly write to fs
    let output_code_dart_api = generator::dart_api::generate(&ir_pack)?;
    let output_code_wire = generator::wire::generate(&ir_pack)?;
    let output_code = output_code_dart_api.merge(output_code_wire);
    output_code.write()?;

    Ok(())
}
