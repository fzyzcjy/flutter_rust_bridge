use crate::codegen::hir::hierarchical::module::HirVisibility;

impl From<&syn::Visibility> for HirVisibility {
    fn from(value: &syn::Visibility) -> Self {
        match value {
            syn::Visibility::Public(_) => HirVisibility::Public,
            syn::Visibility::Restricted(_) => HirVisibility::Restricted,
            syn::Visibility::Inherited => HirVisibility::Inherited,
        }
    }
}
