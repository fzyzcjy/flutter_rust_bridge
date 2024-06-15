pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod spec_generator;
mod text_generator;

use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::misc::path_texts::PathTexts;
use crate::codegen::ir::mir::pack::MirPack;
use anyhow::Result;

pub(crate) struct GeneratorApiDartOutput {
    pub output_texts: PathTexts,
    pub needs_freezed: bool,
}

pub(crate) fn generate(
    mir_pack: &MirPack,
    config: &GeneratorApiDartInternalConfig,
    dumper: &Dumper,
) -> Result<GeneratorApiDartOutput> {
    let spec = spec_generator::generate(mir_pack, config, dumper)?;

    (dumper.with_content(ConfigDumpContent::GeneratorSpec)).dump("api_dart.json", &spec)?;

    let text = text_generator::generate(&spec, config)?;
    (dumper.with_content(ConfigDumpContent::GeneratorText)).dump_path_texts(
        "api_dart",
        &text.output_texts,
        &config.dart_decl_base_output_path,
    )?;

    Ok(GeneratorApiDartOutput {
        output_texts: text.output_texts,
        needs_freezed: spec.namespaced_items.values().any(|x| x.needs_freezed),
    })
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::config::MetaConfig;
    use crate::codegen::config::internal_config::InternalConfig;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::generator::api_dart::generate;
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::Config;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::{get_test_fixture_dir, text_golden_test};
    use serial_test::serial;
    use std::collections::HashMap;
    use std::env;

    #[test]
    #[serial]
    fn test_simple() -> anyhow::Result<()> {
        body(
            "library/codegen/generator/api_dart/mod/simple",
            HashMap::from([
                ("api.dart", "expect_output.dart"),
                ("dep.dart", "expect_output2.dart"),
                ("frb_generated.dart", "expect_output3.dart"),
            ]),
        )
    }

    #[test]
    #[serial]
    fn test_functions() -> anyhow::Result<()> {
        body(
            "library/codegen/generator/api_dart/mod/functions",
            HashMap::from([
                ("api.dart", "expect_output.dart"),
                ("frb_generated.dart", "expect_output2.dart"),
            ]),
        )
    }

    fn body(fixture_name: &str, expect_outputs: HashMap<&str, &str>) -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);
        env::set_current_dir(&test_fixture_dir)?;

        let config = Config::from_files_auto()?;
        let internal_config = InternalConfig::parse(&config, &MetaConfig { watch: false })?;
        let mir_pack = crate::codegen::parser::parse(
            &internal_config.parser,
            &Dumper::new(&Default::default()),
            &GeneratorProgressBarPack::new(),
        )?;
        let actual = generate(
            &mir_pack,
            &internal_config.generator.api_dart,
            &Dumper::new(&Default::default()),
        )?;

        let output_texts = actual.output_texts;
        assert_eq!(
            output_texts.0.len(),
            expect_outputs.len(),
            "output_texts={output_texts:?}"
        );
        for path_text in output_texts.0 {
            let path = path_text.path.file_name().unwrap().to_str().unwrap();
            let expect_output = expect_outputs.get(path).unwrap();
            let raw_text = (path_text.text)
                .all_code()
                .replace(env!("CARGO_PKG_VERSION"), "{VERSION}");
            text_golden_test(raw_text, &test_fixture_dir.join(expect_output))?;
        }

        Ok(())
    }
}
