use crate::utils::namespace::Namespace;

pub(crate) trait HirComponent {
    fn with_namespace(&self, namespace: Namespace) -> Self;
}
