//! Code generator for `flutter_rust_bridge`

pub(crate) mod config;
mod dumper;
mod generator;
pub(crate) mod ir;
pub(crate) mod parser;
mod polisher;
mod preparer;

use crate::codegen::config::internal_config::InternalConfig;
use crate::codegen::dumper::Dumper;
pub use config::config::{Config, ConfigDump};
pub use config::config_parser::*;
use log::debug;

/// Execute the main code generator
pub fn generate(config: Config) -> anyhow::Result<()> {
    debug!("config={config:?}");

    let internal_config = InternalConfig::parse(&config)?;
    debug!("internal_config={internal_config:?}");
    let dumper = Dumper(&internal_config.dumper);
    dumper.dump_config(&config, &internal_config)?;

    preparer::prepare(&internal_config.preparer)?;

    let ir_pack = parser::parse(&internal_config.parser)?;
    dumper.dump_ir(&ir_pack)?;

    let generator_output = generator::generate(&ir_pack, &internal_config.generator)?;

    generator_output.output_texts.write_to_disk()?;

    polisher::polish(
        &internal_config.polisher,
        generator_output.dart_needs_freezed,
        &generator_output.output_texts.paths(),
    )?;

    Ok(())
}
