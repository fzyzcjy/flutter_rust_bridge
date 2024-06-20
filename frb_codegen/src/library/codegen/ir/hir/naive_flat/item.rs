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
    pub vis: HirNaiveFlatItemVis,
}

#[derive(Clone, Debug, Derivative, Serialize, Eq, PartialEq)]
pub(crate) enum HirNaiveFlatItemVis {
    Public,
    Private,
    /// Not completely public, not completely private
    Others,
}
