use crate::utils::namespace::Namespace;
use proc_macro2::Ident;

pub(crate) trait HirCommon {
    fn with_namespace(&self, namespace: Namespace) -> Self;

    fn ident(&self) -> Ident;
}
