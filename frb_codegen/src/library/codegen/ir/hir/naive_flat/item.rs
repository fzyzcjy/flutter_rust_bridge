use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::utils::namespace::Namespace;
use derivative::Derivative;
use serde::Serialize;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct HirNaiveFlatItem {
    pub meta: HirNaiveFlatItemMeta,
    #[serde(skip_serializing)]
    pub item: syn::Item,
}

#[derive(Clone, Debug, Derivative, Serialize)]
pub(crate) struct HirNaiveFlatItemMeta {
    pub namespace: Namespace,
    pub is_public: bool,
}
