pub(super) mod base;
pub(super) mod class;
mod function;
pub(super) mod info;
pub(crate) mod internal_config;
mod misc;

use crate::codegen::generator::api_dart::base::{ApiDartGenerator, ApiDartGeneratorContext};
use crate::codegen::generator::api_dart::internal_config::GeneratorDartApiInternalConfig;
use crate::codegen::generator::output::dart::DartOutputCode;
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::generator::api_dart::class::ty::ApiDartGeneratorClassTrait;
use anyhow::Result;
use itertools::Itertools;

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorDartApiInternalConfig,
) -> Result<DartOutputCode> {
    let distinct_types = ir_pack.distinct_types(true, true);
    let context = ApiDartGeneratorContext { ir_pack, config };

    let funcs = ir_pack
        .funcs
        .iter()
        .map(|f| function::generate_func(f, context, config.dart_enums_style))
        .map(|func| {
            format!(
                "{}{}\n\n{}",
                func.func_comments, func.func_signature, func.companion_field_signature,
            )
        })
        .join("\n\n");

    let classes = distinct_types
        .iter()
        .filter_map(|ty| ApiDartGenerator::new(ty.clone(), context).generate_class())
        .join("\n\n");

    Ok(DartOutputCode {
        code: format!(
            "abstract class {dart_api_class_name} {{
                {funcs}
            }}

            {classes}
            ",
            dart_api_class_name = config.dart_api_class_name,
        ),
    })
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config::InternalConfig;
    use crate::codegen::generator::api_dart::generate;
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
        let actual = generate(&ir_pack, &internal_config.generator.dart.into())?;

        text_golden_test(actual.code, &test_fixture_dir.join("expect_output.dart"))?;

        Ok(())
    }
}
