use crate::codegen::parser::error::Error;
use crate::codegen::parser::source_graph::modules::{Module, ModuleInfo, ModuleSource, Visibility};
use crate::codegen::parser::ParserResult;
use anyhow::Context;
use cargo_metadata::{Metadata, MetadataCommand, Package};
use std::fs;
use std::path::{Path, PathBuf};
use syn::File;

/// Represents a crate, including a map of its modules, imports, structs and enums.
#[derive(Debug, Clone)]
pub struct Crate {
    name: String,
    manifest_path: PathBuf,
    root_module: Module,
}

impl Crate {
    pub fn parse(manifest_path: &Path) -> ParserResult<Self> {
        let manifest_path = fs::canonicalize(manifest_path)?;
        let metadata = execute_cargo_metadata(&manifest_path)?;

        let root_package = metadata.root_package().context("no root package")?;
        let root_src_file = get_root_src_file(root_package)?;

        let root_src_content = fs::read_to_string(&root_src_file)?;
        let root_src_ast = syn::parse_file(&root_src_content)?;

        let root_module_info = get_root_module_info(root_src_file.clone(), root_src_ast);
        let root_module = Module::parse(root_module_info);

        Ok(Crate {
            name: root_package.name.clone(),
            manifest_path,
            root_module,
        })
    }

    pub fn root_module(&self) -> &Module {
        &self.root_module
    }
}

fn execute_cargo_metadata(manifest_path: &Path) -> anyhow::Result<Metadata> {
    let mut cmd = MetadataCommand::new();
    cmd.manifest_path(manifest_path);
    Ok(cmd.exec()?)
}

fn get_root_src_file(root_package: &Package) -> ParserResult<PathBuf> {
    for attempt_relative_path in ["src/lib.rs", "src/main.rs"] {
        let file = root_package
            .manifest_path
            .parent()
            .unwrap()
            .join(attempt_relative_path);
        if file.exists() {
            return Ok(fs::canonicalize(file).unwrap());
        }
    }
    Err(Error::NoEntryPoint)
}

fn get_root_module_info(root_src_file: PathBuf, root_src_ast: File) -> ModuleInfo {
    ModuleInfo {
        visibility: Visibility::Public,
        file_path: root_src_file,
        module_path: vec!["crate".to_string()],
        source: ModuleSource::File(root_src_ast),
    }
}
