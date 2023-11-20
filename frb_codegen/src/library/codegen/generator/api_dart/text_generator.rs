use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;

pub(super) fn generate(spec: ApiDartOutputSpec) -> anyhow::Result<String> {
    let ApiDartOutputSpec { funcs, classes } = spec;

    Ok(format!(
        "import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

        {funcs}

        {classes}
        ",
        funcs = funcs.join("\n\n"),
        classes = classes.join("\n\n"),
    ))
}
