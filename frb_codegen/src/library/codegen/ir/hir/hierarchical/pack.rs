use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::utils::crate_name::CrateName;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HirPack {
    pub(crate) crates: Vec<HirCrate>,
}

impl HirPack {
    pub(crate) fn get_mut_crate(&mut self, name: &CrateName) -> Option<&mut HirCrate> {
        (self.crates.iter_mut()).find(|x| x.name.namespace() == name.namespace())
    }
}
