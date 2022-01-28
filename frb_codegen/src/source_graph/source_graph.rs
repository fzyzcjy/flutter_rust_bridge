use std::{fmt::Debug, fs, path::PathBuf};

use cargo_metadata::MetadataCommand;
use syn::Ident;

/// Represents a crate, including a map of its imports.
#[derive(Debug)]
pub struct Crate {
    pub name: String,
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
            name: root_package.name.clone(),
            manifest_path: fs::canonicalize(manifest_path).unwrap(),
            root_src_file: root_src_file.clone().into(),
            root_module: Module {
                visibility: Visibility::Public,
                file_path: root_src_file.clone(),
                module_path: vec!["crate".to_string()],
                root: Some(ModuleSource::File(file_ast)),
                scope: None,
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

// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug)]
pub enum Visibility {
    Public,
    Crate,
    Restricted, // Not currently supported
    Inherited, // Usually means private
}

fn syn_vis_to_visibility(vis: &syn::Visibility) -> Visibility {
    match vis {
        syn::Visibility::Public(_) => Visibility::Public,
        syn::Visibility::Crate(_) => Visibility::Crate,
        syn::Visibility::Restricted(_) => Visibility::Restricted,
        syn::Visibility::Inherited => Visibility::Inherited,
    }
}

#[derive(Debug)]
pub struct Import;

#[derive(Debug)]
pub enum ModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}

#[derive(Debug)]
pub struct ModuleScope {
    pub modules: Vec<Module>,
    pub enums: Vec<Ident>,
    pub structs: Vec<Ident>,
    pub imports: Vec<Import>,
}

// TODO: Capture struct and enum visibility
pub struct Module {
    pub visibility: Visibility,
    pub file_path: PathBuf,
    pub module_path: Vec<String>,
    pub root: Option<ModuleSource>,
    pub scope: Option<ModuleScope>,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("visibility", &self.visibility)
            .field("module_path", &self.module_path)
            .field("file_path", &self.file_path)
            .field("root", &"omitted")
            .field("scope", &self.scope)
            .finish()
    }
}

impl Module {
    pub fn is_resolved(&self) -> bool {
        self.scope.is_some() && self.root.is_some()
    }

    pub fn resolve(&mut self) {
        self.resolve_modules();
        self.resolve_imports();
    }

    /// Maps out modules, structs and enums within the scope of this module
    fn resolve_modules(&mut self) {
        let mut scope_modules = Vec::new();
        let mut scope_structs = Vec::new();
        let mut scope_enums = Vec::new();

        let items = match self.root.as_ref().unwrap() {
            ModuleSource::File(file) => &file.items,
            ModuleSource::ModuleInFile(items) => items,
        };

        for item in items.iter() {
            match item {
                syn::Item::Struct(item_struct) => {
                    scope_structs.push(item_struct.ident.clone());
                }
                syn::Item::Enum(item_enum) => {
                    scope_enums.push(item_enum.ident.clone());
                }
                syn::Item::Mod(item_mod) => {
                    let ident = item_mod.ident.clone();

                    let mut module_path = self.module_path.clone();
                    module_path.push(ident.to_string());

                    scope_modules.push(match &item_mod.content {
                        Some(content) => {
                            let mut child_module = Module {
                                visibility: syn_vis_to_visibility(&item_mod.vis),
                                file_path: self.file_path.clone(),
                                module_path,
                                root: Some(ModuleSource::ModuleInFile(content.1.clone())),
                                scope: None,
                            };

                            child_module.resolve();

                            child_module
                        }
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
                                visibility: syn_vis_to_visibility(&item_mod.vis),
                                file_path,
                                module_path,
                                root,
                                scope: None,
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

        self.scope = Some(ModuleScope {
            modules: scope_modules,
            enums: scope_enums,
            structs: scope_structs,
            imports: vec![], // Will be filled in by resolve_imports()
        });
    }

    fn resolve_imports(&mut self) {
        // let mut imports = &self.scope.as_ref().unwrap().imports;
    }
}

// mod asdf {
//     pub mod abc {
//         pub const A: i32 = 1;
//     }
// }

// const B: i32 = asdf::abc::A;
