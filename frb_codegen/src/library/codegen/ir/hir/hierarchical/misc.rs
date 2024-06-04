use crate::utils::namespace::Namespace;

pub(crate) trait HirCommon {
    fn with_namespace(&self, namespace: Namespace) -> Self;
}
