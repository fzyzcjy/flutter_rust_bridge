//! Code generator for `flutter_rust_bridge`

pub(crate) mod config;
mod controller;
pub(crate) mod dumper;
pub(crate) mod generator;
pub(crate) mod ir;
mod misc;
pub(crate) mod parser;
mod polisher;
mod pre_generation_cleaner;
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

    pre_generation_cleaner::clean(&internal_config.polisher)?;
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

#[cfg(test)]
mod tests {
    use super::generate_with_fvm_install_mode;
    use crate::codegen::{Config, MetaConfig};
    use crate::misc::FvmInstallMode;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_pre_generation_cleanup_removes_stale_files_during_generate() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        create_minimal_project(temp_dir.path())?;
        let stale_dart_file = temp_dir
            .path()
            .join("lib")
            .join("src")
            .join("rust")
            .join("stale.dart");
        let stale_rust_file = temp_dir
            .path()
            .join("rust")
            .join("src")
            .join("frb_generated.rs");
        let stale_c_file = temp_dir.path().join("frb_generated.h");
        fs::create_dir_all(stale_dart_file.parent().unwrap())?;
        fs::write(&stale_dart_file, "stale")?;
        fs::write(&stale_rust_file, "stale")?;
        fs::write(&stale_c_file, "stale")?;

        generate_with_fvm_install_mode(
            Config {
                base_dir: Some(temp_dir.path().to_string_lossy().into_owned()),
                rust_input: Some("rust/src/api/**/*.rs".to_owned()),
                dart_output: Some("lib/src/rust".to_owned()),
                c_output: Some("frb_generated.h".to_owned()),
                local: Some(true),
                deps_check: Some(false),
                dart_format: Some(false),
                dart_fix: Some(false),
                rust_format: Some(false),
                build_runner: Some(false),
                auto_upgrade_dependency: Some(false),
                pre_generation_cleanup: Some(true),
                ..Default::default()
            },
            MetaConfig { watch: false },
            FvmInstallMode::Skip,
        )?;

        assert!(!stale_dart_file.exists());
        assert!(stale_rust_file.exists());
        assert!(stale_c_file.exists());
        assert_ne!(fs::read_to_string(&stale_rust_file)?, "stale");
        assert_ne!(fs::read_to_string(&stale_c_file)?, "stale");
        assert!(temp_dir
            .path()
            .join("lib")
            .join("src")
            .join("rust")
            .join("api")
            .join("simple.dart")
            .exists());
        Ok(())
    }

    fn create_minimal_project(root: &Path) -> anyhow::Result<()> {
        let api_dir = root.join("rust").join("src").join("api");
        fs::create_dir_all(&api_dir)?;
        fs::create_dir_all(root.join("lib"))?;

        fs::write(root.join("pubspec.yaml"), "name: cleanup_test\n")?;
        fs::write(
            root.join("rust").join("Cargo.toml"),
            format!(
                "[package]\nname = \"cleanup_test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"cdylib\"]\n\n[dependencies]\nflutter_rust_bridge = {{ path = {:?} }}\n",
                frb_rust_path()
            ),
        )?;
        fs::write(
            root.join("rust").join("src").join("lib.rs"),
            "pub mod api;\n",
        )?;
        fs::write(api_dir.join("mod.rs"), "pub mod simple;\n")?;
        fs::write(
            api_dir.join("simple.rs"),
            "pub fn simple_adder(a: i32, b: i32) -> i32 { a + b }\n",
        )?;
        Ok(())
    }

    fn frb_rust_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("frb_rust")
    }
}
