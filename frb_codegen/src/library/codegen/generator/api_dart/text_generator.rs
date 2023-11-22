use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;
use itertools::Itertools;

pub(super) struct ApiDartOutputText {
    // TODO handle multi file
    pub(super) text: String,
}

pub(super) fn generate(spec: ApiDartOutputSpec) -> anyhow::Result<ApiDartOutputText> {
    let ApiDartOutputSpec { funcs, classes } = spec;
    Ok(ApiDartOutputText {
        text: generate_end_api_text(&classes, &funcs),
    })
}

fn generate_end_api_text(
    classes: &[ApiDartGeneratedClass],
    funcs: &[ApiDartGeneratedFunction],
) -> String {
    format!(
        "// ignore_for_file: invalid_use_of_internal_member

        import '../frb_generated.dart';

        {funcs}

        {classes}
        ",
        funcs = funcs.iter().map(|f| generate_function(f)).join("\n\n"),
        classes = classes.iter().map(|c| c.code).join("\n\n"),
    )
}

fn generate_function(func: &ApiDartGeneratedFunction) -> String {
    let ApiDartGeneratedFunction {
        func_comments,
        func_expr,
        func_impl,
        ..
    } = &func;
    format!("{func_comments}{func_expr} => {func_impl};")
}
