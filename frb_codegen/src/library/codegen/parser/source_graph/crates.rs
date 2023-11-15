use std::path::{Path, PathBuf};
use crate::codegen::parser::ParserResult;
use crate::codegen::parser::source_graph::modules::Module;

/// Represents a crate, including a map of its modules, imports, structs and
/// enums.
#[derive(Debug, Clone)]
pub struct Crate {
    name: String,
    manifest_path: PathBuf,
    root_src_file: PathBuf,
    root_module: Module,
}

impl Crate {
    pub fn parse(manifest_path: &Path) -> ParserResult<Self> {
        todo!()
    }

    pub fn root_module(&self) -> &Module { &self.root_module }
}
