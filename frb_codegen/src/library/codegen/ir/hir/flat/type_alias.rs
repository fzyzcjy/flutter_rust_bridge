use crate::codegen::ir::hir::flat::component::{HirFlatComponentBase, HirFlatComponentTrait};
use crate::codegen::ir::hir::misc::serializers::serialize_syn;
use serde::Serialize;
use syn::Type;

#[derive(Clone, Debug, Serialize)]
pub struct HirFlatTypeAlias {
    pub(crate) base: HirFlatComponentBase,
    pub(crate) ident: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) target: Type,
}

impl HirFlatComponentTrait<String> for HirFlatTypeAlias {
    fn base_mut(&mut self) -> &mut HirFlatComponentBase {
        &mut self.base
    }

    fn sort_key(&self) -> String {
        self.ident.to_string()
    }
}
