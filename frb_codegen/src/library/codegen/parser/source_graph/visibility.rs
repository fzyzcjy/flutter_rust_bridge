use crate::codegen::hir::hierarchical::module::Visibility;

impl From<&syn::Visibility> for Visibility {
    fn from(value: &syn::Visibility) -> Self {
        match value {
            syn::Visibility::Public(_) => Visibility::Public,
            syn::Visibility::Restricted(_) => Visibility::Restricted,
            syn::Visibility::Inherited => Visibility::Inherited,
        }
    }
}
