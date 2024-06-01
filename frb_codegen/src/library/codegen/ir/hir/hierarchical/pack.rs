use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HirPack {
    pub(crate) crates: HashMap<String, HirCrate>,
}
