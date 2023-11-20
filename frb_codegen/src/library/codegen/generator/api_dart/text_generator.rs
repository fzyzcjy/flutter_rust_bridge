use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;

pub(super) fn generate(spec: ApiDartOutputSpec) -> anyhow::Result<String> {
    let ApiDartOutputSpec { funcs, classes } = spec;

    Ok(format!(
        "// ignore_for_file: invalid_use_of_internal_member

        import '../frb_generated.dart';

        {funcs}

        {classes}
        ",
        funcs = funcs.join("\n\n"),
        classes = classes.join("\n\n"),
    ))
}
