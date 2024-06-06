use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use serde::Serialize;
use syn::Type;

#[derive(Clone, Debug, Serialize)]
pub struct HirFlatTypeAlias {
    pub(crate) ident: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) target: Type,
}

impl HirFlatComponent<String> for HirFlatTypeAlias {
    fn sort_key(&self) -> String {
        self.ident.to_string()
    }
}
