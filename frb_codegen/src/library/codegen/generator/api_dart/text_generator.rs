use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;
use itertools::Itertools;

pub(super) fn generate(spec: ApiDartOutputSpec) -> anyhow::Result<String> {
    let ApiDartOutputSpec { funcs, classes } = spec;

    let funcs = generate_functions(funcs);

    Ok(format!(
        "// ignore_for_file: invalid_use_of_internal_member

        import '../frb_generated.dart';

        {funcs}

        {classes}
        ",
        funcs = funcs,
        classes = classes.join("\n\n"),
    ))
}

fn generate_functions(funcs: Vec<ApiDartGeneratedFunction>) -> String {
    funcs
        .iter()
        .map(|func| {
            let ApiDartGeneratedFunction {
                func_comments,
                func_expr,
                func_impl,
            } = &func;
            format!("{func_comments}{func_expr} => {func_impl};")
        })
        .join("\n\n")
}
