// This file is named `crates` not `crate`, because the latter is a Rust keyword

use crate::codegen::parser::source_graph::modules::Module;
use std::path::PathBuf;

/// Represents a crate, including a map of its modules, imports, structs and enums.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Crate {
    name: String,
    manifest_path: PathBuf,
    root_module: Module,
}
