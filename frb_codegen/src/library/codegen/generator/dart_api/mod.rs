pub(super) mod base;
pub(super) mod class;
pub(super) mod decl;
mod function;
pub(crate) mod internal_config;
mod misc;

use crate::codegen::generator::dart_api::base::{DartApiGenerator, DartApiGeneratorContext};
use crate::codegen::generator::dart_api::internal_config::GeneratorDartApiInternalConfig;
use crate::codegen::generator::output::dart::DartOutputCode;
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::generator::dart_api::class::ty::DartApiGeneratorClassTrait;
use anyhow::Result;
use itertools::Itertools;

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorDartApiInternalConfig,
) -> Result<DartOutputCode> {
    let distinct_types = ir_pack.distinct_types(true, true);
    let context = DartApiGeneratorContext { ir_pack, config };

    let funcs = ir_pack
        .funcs
        .iter()
        .map(|f| function::generate_func(f, &context, config.dart_enums_style))
        .map(|func| {
            format!(
                "{}{}\n\n{}",
                func.func_comments, func.func_signature, func.companion_field_signature,
            )
        })
        .join("\n\n");

    let classes = distinct_types
        .iter()
        .filter_map(|ty| DartApiGenerator::new(ty.clone(), context.clone()).generate_class())
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
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::get_test_fixture_dir;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_simple() -> anyhow::Result<()> {
        body("library/codegen/generator/dart_api/mod/simple")
    }

    fn body(fixture_name: &str) -> anyhow::Result<()> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);
        let rust_crate_dir = test_fixture_dir.clone();

        todo!();

        Ok(())
    }
}
