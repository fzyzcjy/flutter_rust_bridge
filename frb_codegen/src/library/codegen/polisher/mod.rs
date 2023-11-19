use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::polisher::add_mod_to_lib::try_add_mod_to_lib;
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::command_run;
use crate::commands::format_rust::format_rust;
use crate::library::commands::dart_build_runner::dart_build_runner;
use crate::library::commands::format_dart::format_dart;
use std::fs;

pub(crate) mod add_mod_to_lib;
pub(crate) mod internal_config;

pub(super) fn polish(config: &PolisherInternalConfig, needs_freezed: bool) -> anyhow::Result<()> {
    execute_try_add_mod_to_lib(config);
    execute_duplicate_c_output(config)?;
    execute_build_runner(needs_freezed, config)?;
    execute_dart_format(config)?;
    execute_rust_format(config)?;
    Ok(())
}

fn execute_build_runner(
    needs_freezed: bool,
    config: &PolisherInternalConfig,
) -> anyhow::Result<()> {
    if !(needs_freezed && config.build_runner) {
        return Ok(());
    }

    dart_build_runner(&config.dart_root)
}

fn execute_dart_format(config: &PolisherInternalConfig) -> anyhow::Result<()> {
    command_run!(
        format_dart[config.dart_format_line_length],
        &dart_output_paths.base_path,
        ?config.dart_decl_output_path,
        (
            config.wasm_enabled,
            dart_output_paths.wasm_path,
            dart_output_paths.io_path,
        ),
        (
            needs_freezed && config.build_runner,
            config.dart_freezed_path(),
        )
    )
}

fn execute_rust_format(config: &PolisherInternalConfig) -> anyhow::Result<()> {
    command_run!(
        format_rust,
        &config.rust_output_path[TargetOrCommon::Common],
        &config.rust_output_path[TargetOrCommon::Io],
        (
            config.wasm_enabled,
            &config.rust_output_path[TargetOrCommon::Wasm],
        )
    )
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
        fs::copy(&config.c_output_path, path)?;
    }
    Ok(())
}
