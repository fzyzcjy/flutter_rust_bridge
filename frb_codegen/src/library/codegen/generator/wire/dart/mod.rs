use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::ir::pack::IrPack;
use crate::command_run;
use crate::library::commands::ffigen::{ffigen, FfigenArgs};
use crate::library::commands::format_dart::format_dart;

mod c_binding;
mod emitter;
pub(crate) mod internal_config;
pub(super) mod spec_generator;
mod text_generator;

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
) -> anyhow::Result<()> {
    let c_binding = c_binding::generate(&context.config, c_file_content)?;
    let spec = spec_generator::generate(context);
    let text = text_generator::generate()?;
    emitter::emit()
}
