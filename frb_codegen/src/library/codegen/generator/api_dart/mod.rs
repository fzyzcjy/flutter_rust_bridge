pub(crate) mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

use crate::codegen::dumper::internal_config::ConfigDumpContent;
use crate::codegen::dumper::Dumper;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::misc::PathTexts;
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;

pub(crate) struct GeneratorApiDartOutput {
    pub output_texts: PathTexts,
    pub needs_freezed: bool,
}

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorApiDartInternalConfig,
    dumper: &Dumper,
) -> Result<GeneratorApiDartOutput> {
    let spec = spec_generator::generate(ir_pack, config, dumper)?;
    dumper.dump(ConfigDumpContent::GeneratorSpec, "api_dart.json", &spec)?;

    let text = text_generator::generate(&spec, config)?;
    dumper.dump_path_texts(
        ConfigDumpContent::GeneratorText,
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
    use crate::codegen::parser::reader::CachedRustReader;
    use crate::codegen::{parser, Config};
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::{get_test_fixture_dir, text_golden_test};
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn test_simple() -> anyhow::Result<()> {
        body("library/codegen/generator/api_dart/mod/simple")
    }

    fn body(fixture_name: &str) -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);
        env::set_current_dir(&test_fixture_dir)?;

        let config = Config::from_files_auto()?;
        let internal_config = InternalConfig::parse(&config, &MetaConfig { watch: false })?;
        let mut cached_rust_reader = CachedRustReader::default();
        let ir_pack = parser::parse(
            &internal_config.parser,
            &mut cached_rust_reader,
            &Dumper(&Default::default()),
            &GeneratorProgressBarPack::new(),
        )?;
        let actual = generate(
            &ir_pack,
            &internal_config.generator.api_dart,
            &Dumper(&Default::default()),
        )?;

        let output_texts = actual.output_texts;
        assert_eq!(output_texts.0.len(), 1);

        let raw_text = (output_texts.0[0].text).replace(env!("CARGO_PKG_VERSION"), "{VERSION}");
        text_golden_test(raw_text, &test_fixture_dir.join("expect_output.dart"))?;

        Ok(())
    }
}
