use crate::codegen::ir::mir::comment::MirComment;
use itertools::Itertools;
use syn::*;

pub(crate) fn convert_ident_str(ty: &Type) -> Option<String> {
    if let Type::Path(TypePath { qself: _, path }) = ty {
        if let Some(PathSegment { ident, .. }) = path.segments.first() {
            return Some(ident.to_string());
        }
    }

    // Unhandled case, return None
    None
}

pub(crate) fn parse_comments(attrs: &[Attribute]) -> Vec<MirComment> {
    attrs
        .iter()
        .filter_map(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue {
                path,
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ..
            }) if path.is_ident("doc") => Some(parse_comment(&lit.value())),
            _ => None,
        })
        .collect()
}

fn parse_comment(input: &str) -> MirComment {
    let input = input.trim_matches('\n');
    MirComment(if input.contains('\n') {
        // Dart's formatter has issues with block comments
        // so we convert them ahead of time.
        let formatted = input
            .split('\n')
            .map(|line| format!("///{line}"))
            .collect_vec()
            .join("\n");
        formatted
    } else {
        format!("///{input}")
    })
}
