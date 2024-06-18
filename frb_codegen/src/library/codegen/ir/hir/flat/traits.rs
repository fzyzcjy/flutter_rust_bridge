use crate::codegen::ir::hir::flat::component::{HirFlatComponentBase, HirFlatComponentTrait};
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::misc::serializers::serialize_vec_syn;
use crate::utils::namespace::NamespacedName;

#[derive(Clone, serde::Serialize, Debug)]
pub struct HirFlatTrait {
    pub(crate) base: HirFlatComponentBase,
    pub(crate) name: NamespacedName,
    #[serde(serialize_with = "serialize_vec_syn")]
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) sources: Vec<HirGenerationSource>,
}

impl HirFlatComponentTrait<NamespacedName> for HirFlatTrait {
    fn base_mut(&mut self) -> &mut HirFlatComponentBase {
        &mut self.base
    }

    fn sort_key(&self) -> NamespacedName {
        self.name.clone()
    }
}
