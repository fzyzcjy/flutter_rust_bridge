use std::fs;
use std::path::{Path, PathBuf};
use crate::codegen::parser::ParserResult;
use crate::codegen::parser::source_graph::modules::{Module, ModuleSource, Visibility};

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
        let mut cmd = MetadataCommand::new();
        cmd.manifest_path(manifest_path);

        let metadata = cmd.exec().unwrap();

        let root_package = metadata.root_package().unwrap();
        let root_src_file = {
            let lib_file = root_package
                .manifest_path
                .parent()
                .unwrap()
                .join("src/lib.rs");
            let main_file = root_package
                .manifest_path
                .parent()
                .unwrap()
                .join("src/main.rs");

            if lib_file.exists() {
                fs::canonicalize(lib_file).unwrap()
            } else if main_file.exists() {
                fs::canonicalize(main_file).unwrap()
            } else {
                return Err(super::Error::NoEntryPoint);
            }
        };

        let source_rust_content = fs::read_to_string(&root_src_file).unwrap();
        let file_ast = syn::parse_file(&source_rust_content).unwrap();

        let mut result = Crate {
            name: root_package.name.clone(),
            manifest_path: fs::canonicalize(manifest_path).unwrap(),
            root_src_file: root_src_file.clone(),
            root_module: Module {
                visibility: Visibility::Public,
                file_path: root_src_file,
                module_path: vec!["crate".to_string()],
                source: ModuleSource::File(file_ast),
                scope: None,
            },
        };

        result.resolve();

        Ok(result)
    }

    /// Create a map of the modules for this crate
    fn resolve(&mut self) {
        self.root_module.resolve();
    }

    pub fn root_module(&self) -> &Module { &self.root_module }
}
