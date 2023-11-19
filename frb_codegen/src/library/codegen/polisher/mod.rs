use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::library::commands::dart_build_runner::dart_build_runner;

pub(crate) mod internal_config;

pub(super) fn polish(config: &PolisherInternalConfig, needs_freezed: bool) -> anyhow::Result<()> {
    execute_build_runner(needs_freezed, config)?;
    execute_dart_format(config)?;
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
    todo!()
    // command_run!(
    //     format_dart[config.dart_format_line_length],
    //     &dart_output_paths.base_path,
    //     ?config.dart_decl_output_path,
    //     (
    //         config.wasm_enabled,
    //         dart_output_paths.wasm_path,
    //         dart_output_paths.io_path,
    //     ),
    //     (
    //         needs_freezed && config.build_runner,
    //         config.dart_freezed_path(),
    //     )
    // )
}
