use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::utils::crate_name::CrateName;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct HirPack {
    pub(crate) crates: Vec<HirCrate>,
}
