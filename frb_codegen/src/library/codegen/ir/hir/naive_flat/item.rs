use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
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
    pub source: HirGenerationSource,
    pub is_public: bool,
}
