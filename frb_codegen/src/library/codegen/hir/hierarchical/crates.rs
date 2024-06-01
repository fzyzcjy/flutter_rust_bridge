// This file is named `crates` not `crate`, because the latter is a Rust keyword

use crate::codegen::hir::hierarchical::module::Module;
use std::path::PathBuf;

/// Represents a crate, including a map of its modules, imports, structs and enums.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Crate {
    pub(crate) name: String,
    pub(crate) manifest_path: PathBuf,
    pub(crate) root_module: Module,
}
