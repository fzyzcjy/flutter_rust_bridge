use crate::codegen::ir::hir::flat::component::{HirFlatComponentBase, HirFlatComponentTrait};
use crate::codegen::ir::hir::misc::serializers::serialize_syn;

#[derive(Clone, serde::Serialize, Debug)]
pub struct HirFlatTraitImpl {
    pub(crate) base: HirFlatComponentBase,
    pub(crate) trait_name: String,
    #[serde(serialize_with = "serialize_syn")]
    pub(crate) impl_ty: syn::Type,
}

impl HirFlatComponentTrait<String> for HirFlatTraitImpl {
    fn base_mut(&mut self) -> &mut HirFlatComponentBase {
        &mut self.base
    }

    fn sort_key(&self) -> String {
        self.trait_name.clone()
    }
}
