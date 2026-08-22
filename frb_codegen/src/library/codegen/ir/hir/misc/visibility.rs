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

#[cfg(test)]
mod tests {
    use super::*;

    /// Converts public, restricted, and inherited syntax visibility.
    #[test]
    fn converts_each_syn_visibility_variant() {
        let public: syn::Visibility = syn::parse_str("pub").unwrap();
        let restricted: syn::Visibility = syn::parse_str("pub(crate)").unwrap();
        let inherited: syn::Visibility = syn::parse_str("").unwrap();

        assert_eq!(HirVisibility::from(&public), HirVisibility::Public);
        assert_eq!(HirVisibility::from(&restricted), HirVisibility::Restricted);
        assert_eq!(HirVisibility::from(&inherited), HirVisibility::Inherited);
    }
}
