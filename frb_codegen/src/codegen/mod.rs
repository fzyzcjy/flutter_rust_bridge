//! Code generator for `flutter_rust_bridge`

use clap::ValueEnum;
use serde::Deserialize;

/// Execute the main code generator
pub fn generate(config: &Config) -> anyhow::Result<()> {
    todo!("generate")
}

/// Parsed configuration for code generation
pub struct Config {
    // TODO
}
// TODO

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, ValueEnum, enum_iterator::Sequence)]
#[serde(rename_all = "snake_case")]
pub enum ConfigDump {
    Config,
    Ir,
}

