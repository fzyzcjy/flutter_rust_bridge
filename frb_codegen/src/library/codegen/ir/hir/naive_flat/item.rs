use crate::utils::namespace::Namespace;
use derivative::Derivative;
use serde::Serialize;
use crate::codegen::ir::hir::flat::source::HirFlatGenerationSource;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct HirNaiveFlatItem {
    pub meta: HirNaiveFlatItemMeta,
    #[serde(skip_serializing)]
    pub item: syn::Item,
}

#[derive(Clone, Debug, Derivative, Serialize)]
pub(crate) struct HirNaiveFlatItemMeta {
    pub namespace: Namespace,
    pub source: HirFlatGenerationSource,
    pub is_public: bool,
}
