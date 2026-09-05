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

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    /// Returns the first path segment for simple and qualified path types.
    #[test]
    fn convert_ident_str_uses_first_path_segment() {
        assert_eq!(
            convert_ident_str(&parse_quote!(crate::model::Value)),
            Some("crate".into())
        );
        assert_eq!(
            convert_ident_str(&parse_quote!(Value)),
            Some("Value".into())
        );
    }

    /// Rejects non-path types that cannot name an alias target.
    #[test]
    fn convert_ident_str_rejects_non_path_types() {
        assert_eq!(convert_ident_str(&parse_quote!((u8, u16))), None);
    }

    /// Converts single-line and multiline Rust documentation to Dart comments.
    #[test]
    fn parse_comments_preserves_documentation_shape() {
        let attrs: Vec<Attribute> = vec![
            parse_quote!(#[doc = " One line "]),
            parse_quote!(#[doc = "first\nsecond"]),
        ];

        assert_eq!(
            parse_comments(&attrs)
                .into_iter()
                .map(|comment| comment.0)
                .collect::<Vec<_>>(),
            vec!["/// One line ", "///first\n///second"],
        );
    }

    /// Ignores attributes that are not string-valued documentation.
    #[test]
    fn parse_comments_ignores_non_documentation_attributes() {
        let attrs: Vec<Attribute> =
            vec![parse_quote!(#[derive(Clone)]), parse_quote!(#[doc(hidden)])];

        assert!(parse_comments(&attrs).is_empty());
    }
}
