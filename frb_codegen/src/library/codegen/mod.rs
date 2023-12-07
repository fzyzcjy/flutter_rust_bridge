//! Code generator for `flutter_rust_bridge`

pub(crate) mod config;
pub(crate) mod dumper;
mod generator;
pub(crate) mod ir;
pub(crate) mod misc;
pub(crate) mod parser;
mod polisher;
mod preparer;

use crate::codegen::config::internal_config::InternalConfig;
use crate::codegen::dumper::internal_config::ConfigDumpContent::Config as ContentConfig;
use crate::codegen::dumper::Dumper;
use crate::codegen::parser::reader::CachedRustReader;
pub use config::config::Config;
pub use config::config_parser::*;
pub use dumper::internal_config::ConfigDumpContent;
use log::debug;

/// Execute the main code generator
pub fn generate(config: Config) -> anyhow::Result<()> {
    debug!("config={config:?}");

    let internal_config = InternalConfig::parse(&config)?;
    debug!("internal_config={internal_config:?}");

    let dumper = Dumper(&internal_config.dumper);
    dumper.dump(ContentConfig, "config.json", &config)?;
    dumper.dump(ContentConfig, "internal_config.json", &internal_config)?;

    preparer::prepare(&internal_config.preparer)?;

    let mut cached_rust_reader = CachedRustReader::default();

    let ir_pack = parser::parse(&internal_config.parser, &mut cached_rust_reader, &dumper)?;
    dumper.dump(ConfigDumpContent::Ir, "ir_pack.json", &ir_pack)?;

    let generator_output = generator::generate(&ir_pack, &internal_config.generator, &dumper)?;

    generator_output.output_texts.write_to_disk()?;

    polisher::polish(
        &internal_config.polisher,
        generator_output.dart_needs_freezed,
        &generator_output.output_texts.paths(),
    )?;

    Ok(())
}
