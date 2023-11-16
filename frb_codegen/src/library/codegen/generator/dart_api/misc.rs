use crate::codegen::ir::annotation::IrDartAnnotation;
use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::import::IrDartImport;
use crate::codegen::ir::ty::IrType::Optional;
use convert_case::{Case, Casing};
use itertools::Itertools;

/// A trailing newline is included if comments is not empty.
pub(crate) fn generate_dart_comments(comments: &[IrComment]) -> String {
    let mut comments = comments
        .iter()
        .map(|comment| comment.0)
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
        .collect::<Vec<_>>()
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

pub(crate) fn generate_field_required_modifier(field: &IrField) -> &str {
    if field.default.is_some() || matches!(field, Optional(_)) {
        ""
    } else {
        "required "
    }
}
