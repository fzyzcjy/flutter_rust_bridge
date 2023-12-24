use crate::codegen::dumper::Dumper;
use crate::codegen::parser::reader::CachedRustReader;
use crate::codegen::parser::source_graph::modules::{Module, ModuleInfo, ModuleSource, Visibility};
use crate::library::commands::cargo_metadata::execute_cargo_metadata;
use anyhow::{bail, Context};
use cargo_metadata::Package;
use log::debug;
use std::fs;
use std::path::{Path, PathBuf};
use syn::File;

/// Represents a crate, including a map of its modules, imports, structs and enums.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Crate {
    name: String,
    manifest_path: PathBuf,
    root_module: Module,
}

impl Crate {
    pub(crate) fn parse(
        manifest_path: &Path,
        cached_rust_reader: &mut CachedRustReader,
        dumper: &Dumper,
    ) -> anyhow::Result<Self> {
        debug!("parse manifest_path={manifest_path:?}");

        let manifest_path = fs::canonicalize(manifest_path)?;
        let metadata = execute_cargo_metadata(&manifest_path)?;

        let root_package = metadata.root_package().context("no root package")?;
        let root_src_file = get_root_src_file(root_package)?;

        let root_src_content = fs::read_to_string(&root_src_file)?;
        let root_src_ast = syn::parse_file(&root_src_content)?;

        let root_module_info = get_root_module_info(root_src_file, root_src_ast);
        let root_module = Module::parse(root_module_info, cached_rust_reader, dumper)?;

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

fn get_root_src_file(root_package: &Package) -> anyhow::Result<PathBuf> {
    for attempt_relative_path in ["src/lib.rs", "src/main.rs"] {
        let file = root_package
            .manifest_path
            .parent()
            .unwrap()
            .join(attempt_relative_path);
        if file.exists() {
            return Ok(fs::canonicalize(file).unwrap());
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
        }
    }
    bail!("No src/lib.rs or src/main.rs found for the specified/inferred Cargo.toml.")
    // frb-coverage:ignore-end
}

fn get_root_module_info(root_src_file: PathBuf, root_src_ast: File) -> ModuleInfo {
    ModuleInfo {
        visibility: Visibility::Public,
        file_path: root_src_file,
        module_path: vec!["crate".to_string()],
        source: ModuleSource::File(root_src_ast),
    }
}

#[cfg(test)]
mod tests {
    // tests are in `codegen::parser::mod`
}
