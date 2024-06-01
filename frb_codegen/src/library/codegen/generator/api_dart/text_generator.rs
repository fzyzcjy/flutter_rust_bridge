use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::generator::api_dart::spec_generator::{
    ApiDartOutputSpec, ApiDartOutputSpecItem,
};
use crate::codegen::generator::misc::{generate_code_header, PathText, PathTexts};
use crate::codegen::ir::mir::namespace::Namespace;
use crate::utils::basic_code::DartBasicHeaderCode;
use anyhow::ensure;
use itertools::Itertools;
use std::path::{Path, PathBuf};

pub(super) struct ApiDartOutputText {
    pub(super) output_texts: PathTexts,
}

pub(super) fn generate(
    spec: &ApiDartOutputSpec,
    config: &GeneratorApiDartInternalConfig,
) -> anyhow::Result<ApiDartOutputText> {
    let common_namespace_prefix = Namespace::compute_common_prefix(&spec.namespaced_items.keys().collect());
    let path_texts = PathTexts(
        spec.namespaced_items
            .iter()
            .map(|(namespace, item)| {
                let dart_output_path =
                    compute_path_from_namespace(&config.dart_decl_base_output_path, namespace);
                let text = generate_end_api_text(namespace, &dart_output_path, item)?;
                Ok(PathText::new(dart_output_path, text))
            })
            .collect::<anyhow::Result<Vec<_>>>()?,
    );

    Ok(ApiDartOutputText {
        output_texts: path_texts,
    })
}

fn generate_end_api_text(
    namespace: &Namespace,
    dart_output_path: &Path,
    item: &ApiDartOutputSpecItem,
) -> anyhow::Result<String> {
    let funcs = item
        .funcs
        .iter()
        .sorted_by_key(|f| f.src_lineno_pseudo)
        .map(generate_function)
        .join("\n\n");
    let classes = item.classes.iter().map(|c| c.code.clone()).join("\n\n");

    let path_chunks_len = namespace.path().len();
    ensure!(
        path_chunks_len >= 2,
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        "Please do not put structs in `lib.rs`",
        // frb-coverage:ignore-end
    );
    // TODO use relative path calculation
    let path_frb_generated = "../".repeat(path_chunks_len - 2) + "frb_generated.dart";

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

    let unused_types = (item.unused_types.iter().sorted())
        .map(|t| {
            format!("// The type `{t}` is not used by any `pub` functions, thus it is ignored.\n")
        })
        .join("");

    let skipped_functions = if item.skipped_functions.is_empty() {
        "".to_owned()
    } else {
        format!(
            "// The functions {} are not `pub`, thus are ignored.\n",
            (item.skipped_functions.iter().map(|x| format!("`{x}`"))).join(", "),
        )
    };

    Ok(format!(
        "
        {header}

        {unused_types}{skipped_functions}

        {funcs}

        {classes}
        ",
    ))
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
