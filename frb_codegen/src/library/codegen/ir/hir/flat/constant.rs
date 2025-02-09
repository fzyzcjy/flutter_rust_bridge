use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use crate::utils::namespace::Namespace;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct HirFlatConstant {
    pub(crate) namespace: Namespace,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) item_const: syn::ItemConst,
}

impl HirFlatComponent<String> for HirFlatConstant {
    fn sort_key(&self) -> String {
        self.item_const.ident.to_string()
    }
}
