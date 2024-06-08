//! Code generator for `flutter_rust_bridge`

pub(crate) mod config;
mod controller;
pub(crate) mod dumper;
pub(crate) mod generator;
pub(crate) mod ir;
mod misc;
pub(crate) mod parser;
mod polisher;
mod preparer;

use crate::codegen::config::internal_config::InternalConfig;
use crate::codegen::dumper::internal_config::ConfigDumpContent::Config as ContentConfig;
use crate::codegen::dumper::Dumper;
use crate::codegen::misc::GeneratorProgressBarPack;
pub use config::config::{Config, MetaConfig};
pub use dumper::internal_config::ConfigDumpContent;
use log::debug;

/// Execute the main code generator
pub fn generate(config: Config, meta_config: MetaConfig) -> anyhow::Result<()> {
    debug!("config={config:?} meta_config={meta_config:?}");

    let internal_config = InternalConfig::parse(&config, &meta_config)?;
    debug!("internal_config={internal_config:?}");

    let dumper = Dumper::new(&internal_config.dumper);
    dumper
        .with_content(ContentConfig)
        .dump("config.json", &config)?;

    controller::run(&internal_config.controller, &|| {
        generate_once(&internal_config, &dumper)
    })?;

    Ok(())
}

fn generate_once(internal_config: &InternalConfig, dumper: &Dumper) -> anyhow::Result<()> {
    let progress_bar_pack = GeneratorProgressBarPack::new();

    dumper
        .with_content(ContentConfig)
        .dump("internal_config.json", &internal_config)?;

    preparer::prepare(&internal_config.preparer)?;

    let pb = progress_bar_pack.parse.start();
    let mir_pack = parser::parse(&internal_config.parser, dumper, &progress_bar_pack)?;
    drop(pb);

    let pb = progress_bar_pack.generate.start();
    let generator_output = generator::generate(
        &mir_pack,
        &internal_config.generator,
        dumper,
        &progress_bar_pack,
    )?;
    drop(pb);

    generator_output.output_texts.write_to_disk()?;

    let pb = progress_bar_pack.polish.start();
    polisher::polish(
        &internal_config.polisher,
        generator_output.dart_needs_freezed,
        &generator_output.output_texts.paths(),
        &progress_bar_pack,
    )?;
    drop(pb);

    println!("Done!");

    Ok(())
}
