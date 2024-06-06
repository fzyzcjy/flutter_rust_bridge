use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::utils::namespace::NamespacedName;

#[derive(Clone, serde::Serialize, Debug)]
pub struct HirFlatTrait {
    pub(crate) name: NamespacedName,
}

impl HirFlatComponent<NamespacedName> for HirFlatTrait {
    fn sort_key(&self) -> NamespacedName {
        self.name.clone()
    }
}
