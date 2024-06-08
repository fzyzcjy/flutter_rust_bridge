use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::misc::path_texts::PathTexts;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::misc::GeneratorProgressBarPack;
use std::path::PathBuf;

pub(crate) mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

pub(crate) struct GeneratorWireDartOutput {
    pub output_texts: PathTexts,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
    rust_extern_funcs: &[ExternFunc],
    rust_content_hash: i32,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<GeneratorWireDartOutput> {
    let spec = spec_generator::generate(
        context,
        c_file_content,
        api_dart_actual_output_paths,
        rust_extern_funcs,
        rust_content_hash,
        dumper,
        progress_bar_pack,
    )?;
    (dumper.with_content(ConfigDumpContent::GeneratorSpec)).dump("wire_dart.json", &spec)?;

    let text = text_generator::generate(&spec, context.config)?;
    (dumper.with_content(ConfigDumpContent::GeneratorText)).dump_acc(
        "wire_dart",
        "dart",
        &text.text.clone().map(|x, _| x.map(|x| x.all_code())),
    )?;

    Ok(GeneratorWireDartOutput {
        output_texts: PathTexts::new_from_targets(
            &context.config.dart_impl_output_path,
            &text.text,
        ),
    })
}
