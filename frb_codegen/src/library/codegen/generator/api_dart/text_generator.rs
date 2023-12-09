use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::{
    ApiDartOutputSpec, ApiDartOutputSpecItem,
};
use crate::codegen::generator::misc::{generate_code_header, PathText, PathTexts};
use crate::codegen::ir::namespace::Namespace;
use crate::utils::basic_code::DartBasicHeaderCode;
use itertools::Itertools;
use std::path::{Path, PathBuf};

pub(super) struct ApiDartOutputText {
    pub(super) output_texts: PathTexts,
}

pub(super) fn generate(
    spec: &ApiDartOutputSpec,
    config: &GeneratorApiDartInternalConfig,
) -> anyhow::Result<ApiDartOutputText> {
    let path_texts = PathTexts(
        spec.namespaced_items
            .iter()
            .map(|(namespace, item)| {
                let dart_output_path =
                    compute_path_from_namespace(&config.dart_decl_base_output_path, namespace);
                let text = generate_end_api_text(namespace, &dart_output_path, item);
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
    item: &ApiDartOutputSpecItem,
) -> String {
    let funcs = item
        .funcs
        .iter()
        .sorted_by_key(|f| f.src_lineno)
        .map(generate_function)
        .join("\n\n");
    let classes = item.classes.iter().map(|c| c.code.clone()).join("\n\n");

    // TODO use relative path calculation
    let path_frb_generated = "../".repeat(namespace.path().len() - 2) + "frb_generated.dart";

    let mut header = DartBasicHeaderCode {
        file_top: generate_code_header()
            + "\n\n// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import\n",
        import: format!(
            "
            import '{path_frb_generated}';
            import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
            "
        ),
        ..Default::default()
    };

    if item.needs_freezed {
        header += DartBasicHeaderCode {
            import: "import 'package:freezed_annotation/freezed_annotation.dart' hide protected;"
                .into(),
            part: format!(
                "part '{name}.freezed.dart';",
                name = dart_output_path.file_stem().unwrap().to_str().unwrap()
            ),
            ..Default::default()
        };
    }

    header += item.imports.clone();

    for f in &item.funcs {
        header += f.header.clone();
    }
    for c in &item.classes {
        header += c.header.clone();
    }

    let header = header.all_code();

    format!(
        "
        {header}

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
