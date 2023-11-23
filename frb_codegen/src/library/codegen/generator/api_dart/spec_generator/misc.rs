use crate::codegen::ir::annotation::IrDartAnnotation;
use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::import::IrDartImport;
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::{DistinctTypeGatherer, IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::library::codegen::ir::ty::IrTypeTrait;
use crate::utils::basic_code::DartBasicHeaderCode;
use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use itertools::Itertools;
use pathdiff::diff_paths;

/// A trailing newline is included if comments is not empty.
pub(crate) fn generate_dart_comments(comments: &[IrComment]) -> String {
    let mut comments = comments
        .iter()
        .map(|comment| comment.0.clone())
        .collect_vec()
        .join("\n");
    if !comments.is_empty() {
        comments.push('\n');
    }
    comments
}

pub(crate) fn generate_dart_metadata(metadata: &[IrDartAnnotation]) -> String {
    let mut metadata = metadata
        .iter()
        .map(|it| match &it.library {
            Some(IrDartImport {
                alias: Some(alias), ..
            }) => format!("@{}.{}", alias, it.content),
            _ => format!("@{}", it.content),
        })
        .collect_vec()
        .join("\n");
    if !metadata.is_empty() {
        metadata.push('\n');
    }
    metadata
}

pub(crate) fn generate_dart_maybe_implements_exception(is_exception: bool) -> &'static str {
    if is_exception {
        "implements FrbException"
    } else {
        ""
    }
}

pub(crate) fn generate_function_dart_return_type(func_mode: &IrFuncMode, inner: &str) -> String {
    match func_mode {
        IrFuncMode::Normal => format!("Future<{inner}>"),
        IrFuncMode::Sync => inner.to_string(),
        IrFuncMode::Stream { .. } => format!("Stream<{inner}>"),
    }
}

pub(super) fn generate_imports_which_types_and_funcs_use(
    current_file_namespace: &Namespace,
    types: &Option<&Vec<&IrType>>,
    funcs: &Option<&Vec<&IrFunc>>,
    ir_pack: &IrPack,
) -> anyhow::Result<DartBasicHeaderCode> {
    let mut gatherer = DistinctTypeGatherer::new();
    if let Some(types) = types {
        (types.iter()).for_each(|x| x.visit_types(&mut |ty| gatherer.add(ty), ir_pack));
    }
    if let Some(funcs) = funcs {
        (funcs.iter()).for_each(|x| x.visit_types(&mut |ty| gatherer.add(ty), true, true, ir_pack));
    }
    let interest_types = gatherer.gather();

    let import = interest_types
        .iter()
        .filter_map(|ty| ty.self_namespace())
        .filter(|import_ty_namespace| import_ty_namespace != current_file_namespace)
        .map(|import_ty_namespace| {
            let path_diff = diff_paths(
                import_ty_namespace.to_pseudo_io_path("dart"),
                (current_file_namespace.to_pseudo_io_path("dart").parent()).unwrap(),
            )
            .context("cannot diff path")?;
            Ok(format!(
                "import '{}';\n",
                path_to_string(&path_diff).unwrap()
            ))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .iter()
        .join("");

    Ok(DartBasicHeaderCode {
        import,
        ..Default::default()
    })
}
