use serde::Serialize;

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum HirVisibility {
    Public,
    Restricted,
    Inherited, // Usually means private
}
