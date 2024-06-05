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
