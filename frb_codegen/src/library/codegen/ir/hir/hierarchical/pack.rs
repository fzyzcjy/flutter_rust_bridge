use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use std::collections::HashMap;
use crate::utils::crate_name::CrateName;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HirPack {
    pub(crate) crates: HashMap<CrateName, HirCrate>,
}
