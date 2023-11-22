use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::PathTexts;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;

pub(crate) mod internal_config;
pub(super) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireDartOutput {
    pub output_texts: PathTexts,
    pub dart_needs_freezed: bool,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
    dumper: &Dumper,
) -> anyhow::Result<GeneratorWireDartOutput> {
    let spec = spec_generator::generate(context, c_file_content)?;
    dumper.dump(ConfigDumpContent::Spec, "dump_wire_dart.json", &spec)?;
    let text = text_generator::generate(&spec, &context.config)?;

    Ok(GeneratorWireDartOutput {
        output_texts: PathTexts::new_from_targets(
            &context.config.dart_impl_output_path,
            &text.text,
        ),
        dart_needs_freezed: spec.misc.needs_freezed,
    })
}
