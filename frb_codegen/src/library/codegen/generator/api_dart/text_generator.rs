use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;

pub(super) fn generate(
    spec: ApiDartOutputSpec,
    config: &GeneratorApiDartInternalConfig,
) -> anyhow::Result<String> {
    let ApiDartOutputSpec { funcs, classes } = spec;

    Ok(format!(
        "abstract class {dart_api_class_name} {{
            {funcs}
        }}

        {classes}
        ",
        dart_api_class_name = config.dart_api_class_name,
    ))
}
