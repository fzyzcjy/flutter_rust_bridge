use std::{fmt::Debug, fs, path::PathBuf};

use cargo_metadata::MetadataCommand;

/// Represents a crate, including a map of its imports.
#[derive(Debug)]
pub struct Crate {
    pub manifest_path: PathBuf,
    pub root_src_file: PathBuf,
    pub root_module: Module,
}

impl Crate {
    pub fn new(manifest_path: &String) -> Self {
        let mut cmd = MetadataCommand::new();
        cmd.manifest_path(&manifest_path);

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
                panic!("No src/lib.rs or src/main.rs found for this Cargo.toml file");
            }
        };

        let source_rust_content = fs::read_to_string(&root_src_file).unwrap();
        let file_ast = syn::parse_file(&source_rust_content).unwrap();

        let mut result = Crate {
            manifest_path: fs::canonicalize(manifest_path).unwrap(),
            root_src_file: root_src_file.clone().into(),
            root_module: Module {
                file_path: root_src_file.clone(),
                module_path: vec!["crate".to_string()],
                root: Some(ModuleSource::File(file_ast)),
                children: None,
            },
        };

        result.resolve();

        result
    }

    /// Create a map of the modules for this crate
    pub fn resolve(&mut self) {
        self.root_module.resolve();
    }
}

#[derive(Debug)]
pub enum ModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}

pub struct Module {
    pub file_path: PathBuf,
    pub module_path: Vec<String>,
    pub root: Option<ModuleSource>,
    pub children: Option<Vec<Module>>,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("file_path", &self.file_path)
            .field("module_path", &self.module_path)
            .field("root", &"omitted")
            .field("children", &self.children)
            .finish()
    }
}

impl Module {
    pub fn is_resolved(&self) -> bool {
        self.children.is_some() && self.root.is_some()
    }

    pub fn resolve(&mut self) {
        let mut children = Vec::new();

        let items = match self.root.as_ref().unwrap() {
            ModuleSource::File(file) => &file.items,
            ModuleSource::ModuleInFile(items) => items,
        };

        for item in items.iter() {
            match item {
                syn::Item::Mod(item_mod) => {
                    let ident = item_mod.ident.clone();

                    let mut module_path = self.module_path.clone();
                    module_path.push(ident.to_string());

                    children.push(match &item_mod.content {
                        Some(content) => Module {
                            file_path: self.file_path.clone(),
                            module_path,
                            root: Some(ModuleSource::ModuleInFile(content.1.clone())),
                            children: Some(vec![]),
                        },
                        None => {
                            let folder_path =
                                self.file_path.parent().unwrap().join(ident.to_string());
                            let folder_exists = folder_path.exists();

                            let file_path = if folder_exists {
                                folder_path.join("mod.rs")
                            } else {
                                self.file_path
                                    .parent()
                                    .unwrap()
                                    .join(ident.to_string() + ".rs")
                            };

                            let file_exists = file_path.exists();

                            let root = if file_exists {
                                let source_rust_content = fs::read_to_string(&file_path).unwrap();
                                Some(ModuleSource::File(
                                    syn::parse_file(&source_rust_content).unwrap(),
                                ))
                            } else {
                                None
                            };

                            let mut child_module = Module {
                                file_path,
                                module_path,
                                root,
                                children: None,
                            };

                            if file_exists {
                                child_module.resolve();
                            }

                            child_module
                        }
                    });
                }
                _ => {}
            }
        }

        self.children = Some(children);
    }
}

// mod asdf {
//     pub mod abc {
//         pub const A: i32 = 1;
//     }
// }

// const B: i32 = asdf::abc::A;
