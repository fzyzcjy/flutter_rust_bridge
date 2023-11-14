//! Code generator for `flutter_rust_bridge`

mod config;

pub use config::config::{Config, ConfigDump};
pub use config::config_parser::*;

use log::debug;

/// Execute the main code generator
pub fn generate(config: &Config) -> anyhow::Result<()> {
    debug!("config={config:?}");

    todo!("generate")
}
