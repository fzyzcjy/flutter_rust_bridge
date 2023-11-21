use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;
use itertools::Itertools;

pub(super) struct ApiDartOutputText {
    // TODO handle multi file
    pub(super) end_api_text: String,
}

pub(super) fn generate(spec: ApiDartOutputSpec) -> anyhow::Result<ApiDartOutputText> {
    let ApiDartOutputSpec { funcs, classes } = spec;
    let funcs = generate_functions(funcs);
    Ok(ApiDartOutputText {
        end_api_text: generate_end_api_text(classes, funcs),
    })
}

fn generate_end_api_text(classes: Vec<String>, funcs: String) -> String {
    format!(
        "// ignore_for_file: invalid_use_of_internal_member

        import '../frb_generated.dart';

        {funcs}

        {classes}
        ",
        funcs = funcs,
        classes = classes.join("\n\n"),
    )
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
