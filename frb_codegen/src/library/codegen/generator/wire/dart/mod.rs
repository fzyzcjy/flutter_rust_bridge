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

#[cfg(test)]
mod tests {
    use crate::codegen::config::config::MetaConfig;
    use crate::codegen::config::internal_config::InternalConfig;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::generator;
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::Config;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::get_test_fixture_dir;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn test_no_web_fixture_omits_web_outputs_and_references() -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        let test_fixture_dir =
            get_test_fixture_dir("library/codegen/generator/wire/dart/mod/no_web");
        env::set_current_dir(&test_fixture_dir)?;

        let config = Config::from_files_auto()?;
        let internal_config = InternalConfig::parse(&config, &MetaConfig { watch: false })?;
        let mir_pack = crate::codegen::parser::parse(
            &internal_config.parser,
            &Dumper::new(&Default::default()),
            &GeneratorProgressBarPack::new(),
        )?;
        let actual = generator::generate(
            &mir_pack,
            &internal_config.generator,
            &Dumper::new(&Default::default()),
            &GeneratorProgressBarPack::new(),
        )?;

        let output_paths = actual.output_texts.paths();
        assert!(
            !output_paths
                .iter()
                .any(|path| path.file_name().and_then(|name| name.to_str())
                    == Some("frb_generated.web.dart")),
            "output_paths={output_paths:?}"
        );

        let generated_dart = actual
            .output_texts
            .0
            .iter()
            .find(|path_text| {
                path_text.path.file_name().and_then(|name| name.to_str())
                    == Some("frb_generated.dart")
            })
            .expect("frb_generated.dart should be generated")
            .text
            .all_code();

        assert!(generated_dart.contains("import 'frb_generated.io.dart';"));
        assert!(!generated_dart.contains("frb_generated.web.dart"));
        assert!(!generated_dart.contains("webPrefix"));
        assert!(!generated_dart.contains("wasmBindgenName"));

        Ok(())
    }
}
