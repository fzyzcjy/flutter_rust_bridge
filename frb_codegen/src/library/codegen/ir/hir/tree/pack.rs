use crate::codegen::ir::hir::tree::crates::HirTreeCrate;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HirTreePack {
    pub(crate) crates: Vec<HirTreeCrate>,
}
