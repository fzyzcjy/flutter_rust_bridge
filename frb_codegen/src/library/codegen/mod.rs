//! Code generator for `flutter_rust_bridge`

pub(crate) mod config;

pub use config::config::{Config, ConfigDump};
pub use config::config_parser::*;

use log::debug;
use crate::codegen::config::internal_config::InternalConfig;

/// Execute the main code generator
pub fn generate(config: Config) -> anyhow::Result<()> {
    debug!("config={config:?}");
    let internal_config = InternalConfig::parse(config)?;
    debug!("internal_config={internal_config:?}");

    todo!("generate")
}
