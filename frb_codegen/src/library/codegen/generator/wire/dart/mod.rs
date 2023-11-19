use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;

mod emitter;
pub(crate) mod internal_config;
pub(super) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireDartOutput {
    pub dart_needs_freezed: bool,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
) -> anyhow::Result<GeneratorWireDartOutput> {
    let spec = spec_generator::generate(context, c_file_content)?;
    let _text = text_generator::generate()?;
    emitter::emit()?;

    Ok(GeneratorWireDartOutput {
        dart_needs_freezed: spec.misc.needs_freezed,
    })
}
