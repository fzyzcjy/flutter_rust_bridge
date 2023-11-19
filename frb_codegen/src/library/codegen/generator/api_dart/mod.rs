pub(crate) mod internal_config;
pub(crate) mod spec_generator;
mod text_generator;

use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::ir::pack::IrPack;
use anyhow::Result;

pub(crate) fn generate(ir_pack: &IrPack, config: &GeneratorApiDartInternalConfig) -> Result<()> {
    let text = generate_without_emit(ir_pack, config)?;
}

fn generate_without_emit(
    ir_pack: &IrPack,
    config: &GeneratorApiDartInternalConfig,
) -> Result<String> {
    let spec = spec_generator::generate(ir_pack, config)?;
    text_generator::generate(spec, config)
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config::InternalConfig;
    use crate::codegen::generator::api_dart::generate_without_emit;
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
        let internal_config = InternalConfig::parse(config)?;
        let ir_pack = parser::parse(&internal_config.parser)?;
        let actual = generate_without_emit(&ir_pack, &internal_config.generator.api_dart.into())?;

        text_golden_test(actual, &test_fixture_dir.join("expect_output.dart"))?;

        Ok(())
    }
}
