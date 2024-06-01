//! Code generator for `flutter_rust_bridge`

pub(crate) mod config;
mod controller;
pub(crate) mod dumper;
pub(crate) mod generator;
pub(crate) mod hir_parser;
pub(crate) mod ir;
pub(crate) mod mir_parser;
mod misc;
pub(crate) mod parser;
mod polisher;
mod preparer;

use crate::codegen::config::internal_config::InternalConfig;
use crate::codegen::dumper::internal_config::ConfigDumpContent::Config as ContentConfig;
use crate::codegen::dumper::Dumper;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::mir_parser::internal_config::ParserInternalConfig;
use crate::codegen::mir_parser::reader::CachedRustReader;
use crate::codegen::misc::GeneratorProgressBarPack;
pub use config::config::{Config, MetaConfig};
pub use dumper::internal_config::ConfigDumpContent;
use log::debug;
use std::path::Path;

/// Execute the main code generator
pub fn generate(config: Config, meta_config: MetaConfig) -> anyhow::Result<()> {
    debug!("config={config:?} meta_config={meta_config:?}");

    let internal_config = InternalConfig::parse(&config, &meta_config)?;
    debug!("internal_config={internal_config:?}");

    let dumper = Dumper(&internal_config.dumper);
    dumper.dump(ContentConfig, "config.json", &config)?;

    controller::run(&internal_config.controller, &|| {
        generate_once(&internal_config, &dumper)
    })?;

    Ok(())
}

fn generate_once(internal_config: &InternalConfig, dumper: &Dumper) -> anyhow::Result<()> {
    let progress_bar_pack = GeneratorProgressBarPack::new();

    dumper.dump(ContentConfig, "internal_config.json", &internal_config)?;

    preparer::prepare(&internal_config.preparer)?;

    let pb = progress_bar_pack.parse.start();
    let mir_pack = parse(&internal_config.parser, dumper, &progress_bar_pack)?;
    dumper.dump(ConfigDumpContent::Mir, "mir_pack.json", &mir_pack)?;
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

// TODO mv
fn parse(
    config: &ParserInternalConfig,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<MirPack> {
    let pb = progress_bar_pack.parse_read.start();
    let mut cached_rust_reader = CachedRustReader::default();
    let file = cached_rust_reader.read_rust_crate(&config.rust_crate_dir, dumper)?;
    drop(pb);

    let pb = progress_bar_pack.parse_hir.start();
    let hir_hierarchical = hir_parser::hierarchical::parse(config, file)?;
    let hir_flat = hir_parser::flat::parse(&hir_hierarchical.root_module)?;
    drop(pb);

    let pb = progress_bar_pack.parse_mir.start();
    let mir_pack = mir_parser::parse(config, &hir_flat)?;
    drop(pb);

    Ok(mir_pack)
}
