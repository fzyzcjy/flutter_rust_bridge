use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::ir::pack::IrPack;
use crate::command_run;
use crate::library::commands::dart_build_runner::dart_build_runner;
use crate::library::commands::ffigen::{ffigen, FfigenArgs};
use crate::library::commands::format_dart::format_dart;

mod c_binding;
mod emitter;
pub(crate) mod internal_config;
pub(super) mod spec_generator;

pub(crate) fn generate(context: WireDartGeneratorContext) -> anyhow::Result<()> {
    let c_binding = c_binding::generate()?;

    let spec = spec_generator::generate(context);

    execute_build_runner(spec.misc.needs_freezed, &context.config)?;
    execute_dart_format(&context.config)?;

    emitter::emit()
}

fn execute_build_runner(
    needs_freezed: bool,
    config: &GeneratorWireDartInternalConfig,
) -> anyhow::Result<()> {
    if !(needs_freezed && config.build_runner) {
        return Ok(());
    }

    dart_build_runner(&config.dart_root)
}

fn execute_dart_format(config: &GeneratorWireDartInternalConfig) -> anyhow::Result<()> {
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
