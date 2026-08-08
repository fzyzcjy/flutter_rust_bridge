use crate::utils::namespace::Namespace;
use serde::Serialize;

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum HirVisibility {
    Public,
    Restricted,
    Inherited, // Usually means private
}

impl From<&syn::Visibility> for HirVisibility {
    fn from(value: &syn::Visibility) -> Self {
        match value {
            syn::Visibility::Public(_) => HirVisibility::Public,
            syn::Visibility::Restricted(_) => HirVisibility::Restricted,
            syn::Visibility::Inherited => HirVisibility::Inherited,
        }
    }
}

pub(crate) fn is_visibility_accessible_from(
    visibility: &syn::Visibility,
    declaration_namespace: &Namespace,
    access_namespace: &Namespace,
) -> bool {
    match visibility {
        syn::Visibility::Public(_) => true,
        syn::Visibility::Inherited => declaration_namespace.is_prefix_of(access_namespace),
        syn::Visibility::Restricted(restricted) => {
            resolve_restricted_namespace(&restricted.path, declaration_namespace)
                .is_prefix_of(access_namespace)
        }
    }
}

fn resolve_restricted_namespace(path: &syn::Path, declaration_namespace: &Namespace) -> Namespace {
    let segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    let declaration_segments = declaration_namespace
        .path()
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    match segments.first().map(String::as_str) {
        Some("crate") => Namespace::new(
            [
                vec![declaration_segments[0].clone()],
                segments[1..].to_vec(),
            ]
            .concat(),
        ),
        Some("self") => Namespace::new([declaration_segments, segments[1..].to_vec()].concat()),
        Some("super") => {
            let super_count = segments
                .iter()
                .take_while(|segment| segment.as_str() == "super")
                .count();
            Namespace::new(
                [
                    declaration_segments[..declaration_segments.len() - super_count].to_vec(),
                    segments[super_count..].to_vec(),
                ]
                .concat(),
            )
        }
        _ => Namespace::new(segments),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks Rust visibility scopes against a sibling generated module.
    #[test]
    fn test_visibility_accessibility_from_sibling_module() {
        let declaration_namespace = Namespace::new_raw("crate::api::models".to_owned());
        let generated_namespace = Namespace::new_raw("crate::frb_generated".to_owned());

        assert!(is_visibility_accessible_from(
            &syn::parse_quote!(pub),
            &declaration_namespace,
            &generated_namespace,
        ));
        assert!(!is_visibility_accessible_from(
            &syn::Visibility::Inherited,
            &declaration_namespace,
            &generated_namespace,
        ));
        assert!(is_visibility_accessible_from(
            &syn::parse_quote!(pub(crate)),
            &declaration_namespace,
            &generated_namespace,
        ));
        assert!(!is_visibility_accessible_from(
            &syn::parse_quote!(pub(super)),
            &declaration_namespace,
            &generated_namespace,
        ));
        assert!(!is_visibility_accessible_from(
            &syn::parse_quote!(pub(in crate::api)),
            &declaration_namespace,
            &generated_namespace,
        ));
    }
}
