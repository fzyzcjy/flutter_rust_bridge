use crate::utils::namespace::Namespace;

pub(crate) trait HirCommon {
    fn with_namespace(&self, namespace: Namespace) -> Self;

    fn name_for_use_stmt(&self) -> String;
}
