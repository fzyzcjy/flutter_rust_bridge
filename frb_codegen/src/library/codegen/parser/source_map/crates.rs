use std::path::{Path, PathBuf};
use crate::codegen::parser::ParserResult;
use crate::codegen::parser::source_map::modules::Module;

/// Represents a crate, including a map of its modules, imports, structs and
/// enums.
#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub manifest_path: PathBuf,
    pub root_src_file: PathBuf,
    pub root_module: Module,
}

impl Crate {
    pub fn parse(manifest_path: &Path) -> ParserResult<Self> {
        todo!()
    }
}
