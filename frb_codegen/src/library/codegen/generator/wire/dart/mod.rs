use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::PathTexts;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use std::path::PathBuf;

pub(crate) mod internal_config;
pub(super) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireDartOutput {
    pub output_texts: PathTexts,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
    dumper: &Dumper,
) -> anyhow::Result<GeneratorWireDartOutput> {
    let spec = spec_generator::generate(
        context,
        c_file_content,
        api_dart_actual_output_paths,
        dumper,
    )?;
    dumper.dump(ConfigDumpContent::GeneratorSpec, "wire_dart.json", &spec)?;

    let text = text_generator::generate(&spec, &context.config)?;

    Ok(GeneratorWireDartOutput {
        output_texts: PathTexts::new_from_targets(
            &context.config.dart_impl_output_path,
            &text.text,
        ),
    })
}
