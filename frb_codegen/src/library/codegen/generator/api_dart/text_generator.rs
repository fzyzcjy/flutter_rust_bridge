use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::{
    ApiDartOutputSpec, ApiDartOutputSpecItem,
};
use crate::codegen::generator::misc::target::TargetOrCommonMap;
use crate::codegen::generator::misc::{generate_code_header, PathText, PathTexts};
use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::utils::basic_code::DartBasicHeaderCode;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::Namespace;
use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use itertools::{concat, Itertools};
use pathdiff::diff_paths;
use std::path::{Path, PathBuf};

pub(super) struct ApiDartOutputText {
    pub(super) output_texts: PathTexts,
}

pub(super) fn generate(
    spec: &ApiDartOutputSpec,
    config: &GeneratorApiDartInternalConfig,
) -> anyhow::Result<ApiDartOutputText> {
    // let common_namespace_prefix =
    //     Namespace::compute_common_prefix(&spec.namespaced_items.keys().collect_vec());
    // debug!("common_namespace_prefix={common_namespace_prefix:?}");

    let path_texts = PathTexts(
        spec.namespaced_items
            .iter()
            .map(|(namespace, item)| {
                let dart_output_path =
                    compute_path_from_namespace(&config.dart_decl_base_output_path, namespace);
                let text =
                    generate_end_api_text(&dart_output_path, &config.dart_impl_output_path, item)?;
                Ok(PathText::new(dart_output_path, text))
            })
            .collect::<anyhow::Result<Vec<_>>>()?,
    );

    Ok(ApiDartOutputText {
        output_texts: path_texts,
    })
}

fn generate_end_api_text(
    dart_output_path: &Path,
    dart_impl_output_path: &TargetOrCommonMap<PathBuf>,
    item: &ApiDartOutputSpecItem,
) -> anyhow::Result<String> {
    let funcs = item
        .funcs
        .iter()
        .sorted_by_key(|f| f.src_lineno_pseudo)
        .map(generate_function)
        .join("\n\n");
    let classes = item.classes.iter().map(|c| c.code.clone()).join("\n\n");

    let path_frb_generated = diff_paths(
        &dart_impl_output_path.common,
        dart_output_path.parent().unwrap(),
    )
    .with_context(|| "Fail to find relative path".to_string())?;
    let path_frb_generated = path_to_string(&path_frb_generated)?.replace('\\', "/");

    let preamble = &item.preamble.as_str();
    let mut header = DartBasicHeaderCode {
        file_top: generate_code_header()
            + if !preamble.is_empty() {"\n\n"} else {""} + preamble
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

    let skips = compute_skips(item);

    Ok(format!(
        "
        {header}

        {skips}

        {funcs}

        {classes}
        ",
    ))
}

fn compute_skips(item: &ApiDartOutputSpecItem) -> String {
    (item.skips.iter())
        .into_group_map_by(|t| t.reason)
        .into_iter()
        .sorted_by_key(|(reason, _)| *reason)
        .map(|(reason, names)| {
            format!(
                "// {}: {}\n",
                reason.explanation_prefix(),
                (names.iter().map(|x| format!("`{}`", x.name.name)))
                    .sorted()
                    .join(", "),
            )
        })
        .join("")
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
    let raw_path = namespace.path();
    let chunks = match raw_path[0] {
        CrateName::SELF_CRATE => raw_path[1..].to_owned(),
        _ => concat([vec![THIRD_PARTY_DIR_NAME], raw_path.clone()]),
    };

    let ans_without_extension =
        (chunks.iter()).fold(dart_decl_base_output_path.to_owned(), |a, b| a.join(b));
    ans_without_extension.with_extension("dart")
}
