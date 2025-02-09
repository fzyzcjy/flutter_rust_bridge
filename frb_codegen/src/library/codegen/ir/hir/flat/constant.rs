use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use crate::utils::namespace::Namespace;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFlatConstant {
    pub(crate) namespace: Namespace,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) item_const: syn::ItemConst,
}
