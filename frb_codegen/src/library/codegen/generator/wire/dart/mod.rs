use crate::codegen::generator::misc::OutputTexts;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;

pub(crate) mod internal_config;
pub(super) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireDartOutput {
    pub output_texts: OutputTexts,
    pub dart_needs_freezed: bool,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
) -> anyhow::Result<GeneratorWireDartOutput> {
    let spec = spec_generator::generate(context, c_file_content)?;
    let text = text_generator::generate(&spec, &context.config)?;

    Ok(GeneratorWireDartOutput {
        output_texts: todo!(),
        dart_needs_freezed: spec.misc.needs_freezed,
    })
}
