use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::ApiDartOutputSpec;
use crate::codegen::generator::misc::{PathText, PathTexts};
use crate::codegen::ir::namespace::Namespace;
use itertools::Itertools;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(super) struct ApiDartOutputText {
    pub(super) output_texts: PathTexts,
}

pub(super) fn generate(
    spec: &ApiDartOutputSpec,
    config: &GeneratorApiDartInternalConfig,
) -> anyhow::Result<ApiDartOutputText> {
    let ApiDartOutputSpec { funcs, classes, .. } = spec;

    let grouped_funcs = funcs.iter().into_group_map_by(|x| x.namespace.clone());
    let grouped_classes = classes.iter().into_group_map_by(|x| x.namespace.clone());

    let namespaces = grouped_funcs
        .keys()
        .chain(grouped_classes.keys())
        .collect::<HashSet<_>>();

    let path_texts = PathTexts(
        namespaces
            .iter()
            .map(|&namespace| {
                let dart_output_path =
                    compute_path_from_namespace(&config.dart_decl_base_output_path, &namespace);
                let text = generate_end_api_text(
                    namespace,
                    &dart_output_path,
                    &grouped_classes.get(namespace),
                    &grouped_funcs.get(namespace),
                );
                PathText::new(dart_output_path, text)
            })
            .collect_vec(),
    );

    Ok(ApiDartOutputText {
        output_texts: path_texts,
    })
}

fn generate_end_api_text(
    namespace: &Namespace,
    dart_output_path: &Path,
    classes: &Option<&Vec<&ApiDartGeneratedClass>>,
    funcs: &Option<&Vec<&ApiDartGeneratedFunction>>,
) -> String {
    let needs_freezed = classes
        .map(|classes| classes.iter().any(|c| c.needs_freezed))
        .unwrap_or(false);
    let parts = if needs_freezed {
        format!(
            "part '{name}.freezed.dart';",
            name = dart_output_path.file_stem().unwrap().to_str().unwrap()
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

fn compute_path_from_namespace(
    dart_decl_base_output_path: &Path,
    namespace: &Namespace,
) -> PathBuf {
    let chunks = namespace.path_exclude_self_crate();
    let ans_without_extension = chunks
        .iter()
        .fold(dart_decl_base_output_path.to_owned(), |a, b| a.join(b));
    ans_without_extension.with_extension("dart")
}
