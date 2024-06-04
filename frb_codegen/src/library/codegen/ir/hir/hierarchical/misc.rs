use crate::utils::namespace::Namespace;

pub(crate) trait WithNamespace {
    fn with_namespace(&self, namespace: Namespace) -> Self;
}
