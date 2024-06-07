use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use crate::utils::namespace::Namespace;
use derivative::Derivative;
use serde::Serialize;

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct HirNaiveFlatItem {
    pub meta: HirNaiveFlatItemMeta,
    #[serde(serialize_with = "serialize_syn")]
    pub item: syn::Item,
}

#[derive(Clone, Debug, Derivative, Serialize)]
pub(crate) struct HirNaiveFlatItemMeta {
    pub namespace: Namespace,
    pub sources: Vec<HirGenerationSource>,
    pub is_module_public: bool,
}
