// This file is named `crates` not `crate`, because the latter is a Rust keyword

use crate::codegen::ir::hir::hierarchical::module::HirModule;
use std::path::PathBuf;

/// Represents a crate, including a map of its modules, imports, structs and enums.
#[derive(Debug, Clone, serde::Serialize)]
pub struct HirCrate {
    pub(crate) root_module: HirModule,
}
