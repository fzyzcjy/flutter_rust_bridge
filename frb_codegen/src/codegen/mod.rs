//! Code generator for `flutter_rust_bridge`

use clap::{Args};
use serde::Deserialize;

/// Execute the main code generator
pub fn generate(config: &Config) -> anyhow::Result<()> {
    todo!("generate")
}

/// Configuration for code generation
// For internal usage, please use [ParsedConfig] instead.
#[derive(Args, Debug, PartialEq, Eq, Deserialize, Default)]
pub struct Config {
    // TODO
}

// TODO `ParsedConfig`
