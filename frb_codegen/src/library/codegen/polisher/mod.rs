use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::polisher::add_mod_to_lib::try_add_mod_to_lib;
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::commands::format_rust::format_rust;
use crate::library::commands::dart_build_runner::dart_build_runner;
use crate::library::commands::dart_fix::dart_fix;
use crate::library::commands::dart_format::dart_format;
use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use anyhow::Context;
use cargo_metadata::VersionReq;
use itertools::Itertools;
use lazy_static::lazy_static;
use log::warn;
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) mod add_mod_to_lib;
mod auto_upgrade;
pub(crate) mod internal_config;

pub(super) fn polish(
    config: &PolisherInternalConfig,
    needs_freezed: bool,
    needs_json_serializable: bool,
    output_paths: &[PathBuf],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    execute_try_add_mod_to_lib(config);
    execute_duplicate_c_output(config)?;
    ensure_dependencies(config, needs_freezed, needs_json_serializable)?;

    warn_if_fail(
        execute_build_runner(needs_freezed, config, progress_bar_pack),
        "execute_build_runner",
    );
    warn_if_fail(
        execute_dart_fix(config, progress_bar_pack),
        "execute_dart_fix",
    );

    // Even if formatting generated code fails, it is not a big problem, and our codegen should not fail.
    warn_if_fail(
        execute_dart_format(config, output_paths, progress_bar_pack),
        "execute_dart_format",
    );
    warn_if_fail(
        execute_rust_format(output_paths, &config.rust_crate_dir, progress_bar_pack),
        "execute_rust_format",
    );

    warn_if_fail(
        auto_upgrade::execute(
            progress_bar_pack,
            &config.dart_root,
            &config.rust_crate_dir,
            config.enable_auto_upgrade,
        ),
        "auto_upgrade",
    );

    Ok(())
}

fn ensure_dependencies(
    config: &PolisherInternalConfig,
    needs_freezed: bool,
    needs_json_serializable: bool,
) -> anyhow::Result<()> {
    lazy_static! {
        pub(crate) static ref ANY_REQUIREMENT: VersionReq = VersionReq::parse(">= 1.0.0").unwrap();
    }

    if needs_freezed {
        let repo = DartRepository::from_path(&config.dart_root)?;
        repo.has_specified_and_installed("freezed", DartDependencyMode::Dev, &ANY_REQUIREMENT)?;
        repo.has_specified_and_installed(
            "freezed_annotation",
            DartDependencyMode::Main,
            &ANY_REQUIREMENT,
        )?;
        repo.has_specified_and_installed(
            "build_runner",
            DartDependencyMode::Dev,
            &ANY_REQUIREMENT,
        )?;
    }

    if needs_json_serializable {
        let repo = DartRepository::from_path(&config.dart_root)?;
        repo.has_specified_and_installed(
            "json_annotation",
            DartDependencyMode::Main,
            &ANY_REQUIREMENT,
        )?;
        repo.has_specified_and_installed(
            "json_serializable",
            DartDependencyMode::Dev,
            &ANY_REQUIREMENT,
        )?;
    }

    Ok(())
}

fn warn_if_fail(r: anyhow::Result<()>, debug_name: &str) -> bool {
    match r {
        Ok(_) => true,
        Err(_) => {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            warn!("Fail to {debug_name}, but continue to run.\nError details: {r:?}");
            false
            // frb-coverage:ignore-end
        }
    }
}

fn execute_build_runner(
    needs_freezed: bool,
    config: &PolisherInternalConfig,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    if !(needs_freezed && config.build_runner) {
        return Ok(());
    }

    let _pb = progress_bar_pack.polish_dart_build_runner.start();
    dart_build_runner(&config.dart_root)
}

fn execute_dart_fix(
    config: &PolisherInternalConfig,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_dart_fix.start();
    dart_fix(&config.dart_output)
}

fn execute_dart_format(
    config: &PolisherInternalConfig,
    output_paths: &[PathBuf],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_dart_formatter.start();
    dart_format(
        &filter_paths_by_extension(output_paths, "dart"),
        &config.dart_root,
        config.dart_format_line_length,
        &["g.dart", "freezed.dart"],
    )
}

fn execute_rust_format(
    output_paths: &[PathBuf],
    base_path: &Path,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_rust_formatter.start();
    format_rust(&filter_paths_by_extension(output_paths, "rs"), base_path)
}

fn filter_paths_by_extension(paths: &[PathBuf], extension: &str) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|path| path.extension().unwrap().to_str().unwrap() == extension)
        .cloned()
        .collect_vec()
}

fn execute_try_add_mod_to_lib(config: &PolisherInternalConfig) {
    if config.add_mod_to_lib {
        try_add_mod_to_lib(&config.rust_crate_dir, &config.rust_output_path);
    }
}

fn execute_duplicate_c_output(config: &PolisherInternalConfig) -> anyhow::Result<()> {
    for path in config.duplicated_c_output_path.iter() {
        fs::copy(
            (config.c_output_path.as_ref())
                .context("When having duplicated_c_output_path, should also have c_output_path")?,
            path,
        )?;
    }
    Ok(())
}
