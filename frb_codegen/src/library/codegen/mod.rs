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
use crate::misc::FvmInstallMode;
pub use config::config::{Config, MetaConfig};
pub use dumper::internal_config::ConfigDumpContent;
pub use ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use log::debug;

/// Execute the main code generator
pub fn generate(config: Config, meta_config: MetaConfig) -> anyhow::Result<()> {
    // This compatibility wrapper only preserves the public API shape; behavior is covered
    // through the explicit mode-aware generator path.
    // frb-coverage:ignore-start
    generate_with_fvm_install_mode(config, meta_config, FvmInstallMode::Normal)
    // frb-coverage:ignore-end
}

/// Execute the main code generator with an explicit FVM install mode.
pub fn generate_with_fvm_install_mode(
    config: Config,
    meta_config: MetaConfig,
    fvm_install_mode: FvmInstallMode,
) -> anyhow::Result<()> {
    if std::env::var_os(crate::library::commands::cargo_expand::CODEGEN_RUNNING_ENV).is_some() {
        return Ok(());
    }

    debug!("config={config:?} meta_config={meta_config:?}");

    let mut internal_config = InternalConfig::parse(&config, &meta_config)?;
    internal_config.polisher.fvm_install_mode = fvm_install_mode;
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

#[cfg(test)]
mod tests {
    use super::{generate, Config, MetaConfig};
    use crate::library::commands::cargo_expand::CODEGEN_RUNNING_ENV;
    use serial_test::serial;
    use std::ffi::OsString;

    struct RunningGuard(Option<OsString>);

    impl Drop for RunningGuard {
        fn drop(&mut self) {
            match &self.0 {
                Some(value) => std::env::set_var(CODEGEN_RUNNING_ENV, value),
                None => std::env::remove_var(CODEGEN_RUNNING_ENV),
            }
        }
    }

    /// Nested expansion skips generation before parsing even an invalid configuration.
    #[test]
    #[serial]
    fn test_nested_expansion_skips_generation() {
        let _guard = RunningGuard(std::env::var_os(CODEGEN_RUNNING_ENV));
        std::env::set_var(CODEGEN_RUNNING_ENV, "1");

        generate(Config::default(), MetaConfig::default()).unwrap();
    }
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
        generator_output.dart_needs_json_serializable,
        &generator_output.output_texts.paths(),
        &progress_bar_pack,
    )?;
    drop(pb);

    println!("Done!");

    Ok(())
}
