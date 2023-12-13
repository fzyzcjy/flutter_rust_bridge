use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::polisher::add_mod_to_lib::try_add_mod_to_lib;
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::commands::format_rust::format_rust;
use crate::library::commands::dart_build_runner::dart_build_runner;
use crate::library::commands::format_dart::format_dart;
use anyhow::{bail, Context};
use itertools::Itertools;
use log::warn;
use std::fs;
use std::path::PathBuf;

pub(crate) mod add_mod_to_lib;
pub(crate) mod internal_config;

pub(super) fn polish(
    config: &PolisherInternalConfig,
    needs_freezed: bool,
    output_paths: &[PathBuf],
) -> anyhow::Result<()> {
    execute_try_add_mod_to_lib(config);
    execute_duplicate_c_output(config)?;

    warn_if_fail(
        execute_build_runner(needs_freezed, config),
        "execute_build_runner",
    );

    // Even if formatting generated code fails, it is not a big problem, and our codegen should not fail.
    warn_if_fail(
        execute_dart_format(config, output_paths),
        "execute_dart_format",
    );
    warn_if_fail(execute_rust_format(output_paths), "execute_rust_format");

    Ok(())
}

fn warn_if_fail(r: anyhow::Result<()>, debug_name: &str) -> bool {
    match r {
        Ok(_) => true,
        Err(_) => {
            warn!("Fail to {debug_name}, but continue to run.\nError details: {r:?}");
            false
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

fn execute_dart_format(
    config: &PolisherInternalConfig,
    output_paths: &[PathBuf],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_dart_formatter.start();
    format_dart(
        &filter_paths_by_extension(output_paths, "dart"),
        config.dart_format_line_length,
    )
}

fn execute_rust_format(
    output_paths: &[PathBuf],
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_rust_formatter.start();
    format_rust(&filter_paths_by_extension(output_paths, "rs"))
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
        try_add_mod_to_lib(
            &config.rust_crate_dir,
            &config.rust_output_path[TargetOrCommon::Common],
        );
    }
}

fn execute_duplicate_c_output(config: &PolisherInternalConfig) -> anyhow::Result<()> {
    for path in config.duplicated_c_output_path.iter() {
        fs::copy(
            config
                .c_output_path
                .as_ref()
                .context("When having duplicated_c_output_path, should also have c_output_path")?,
            path,
        )?;
    }
    Ok(())
}
