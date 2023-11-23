use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;
use crate::codegen::ir::namespace::Namespace;
use itertools::Itertools;
use std::collections::HashSet;
use std::path::Path;

pub(super) struct ApiDartOutputText {
    pub(super) namespaced_texts: Vec<(Namespace, String)>,
}

pub(super) fn generate(spec: &ApiDartOutputSpec) -> anyhow::Result<ApiDartOutputText> {
    let ApiDartOutputSpec { funcs, classes, .. } = spec;

    let grouped_funcs = funcs.iter().into_group_map_by(|x| x.namespace.clone());
    let grouped_classes = classes.iter().into_group_map_by(|x| x.namespace.clone());

    let namespaces = grouped_funcs
        .keys()
        .chain(grouped_classes.keys())
        .collect::<HashSet<_>>();

    let namespaced_texts = namespaces
        .iter()
        .map(|&namespace| {
            (
                namespace.to_owned(),
                generate_end_api_text(
                    namespace,
                    &grouped_classes.get(namespace),
                    &grouped_funcs.get(namespace),
                    TODO,
                ),
            )
        })
        .collect_vec();

    Ok(ApiDartOutputText { namespaced_texts })
}

fn generate_end_api_text(
    namespace: &Namespace,
    classes: &Option<&Vec<&ApiDartGeneratedClass>>,
    funcs: &Option<&Vec<&ApiDartGeneratedFunction>>,
    dart_output_path: &Path,
) -> String {
    let needs_freezed = classes
        .map(|classes| classes.iter().any(|c| c.needs_freezed))
        .unwrap_or(false);
    let parts = if needs_freezed {
        format!(
            "part '{name}.freezed.dart';",
            name = dart_output_path.file_name().unwrap().to_str().unwrap()
        )
    } else {
        "".to_owned()
    };

    let funcs = funcs
        .map(|funcs| funcs.iter().map(|f| generate_function(f)).join("\n\n"))
        .unwrap_or_default();
    let classes = classes
        .map(|classes| classes.iter().map(|c| c.code.clone()).join("\n\n"))
        .unwrap_or_default();

    // TODO use relative path calculation
    let path_frb_generated = "../".repeat(namespace.path().len() - 2) + "frb_generated.dart";

    format!(
        "// ignore_for_file: invalid_use_of_internal_member

        import '{path_frb_generated}';
        import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

        {parts}

        {funcs}

        {classes}
        ",
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
