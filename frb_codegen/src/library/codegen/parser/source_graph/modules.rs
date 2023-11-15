use crate::codegen::parser::reader::read_rust_file;
use derivative::Derivative;
use log::{debug, warn};
use std::collections::HashMap;
use std::path::PathBuf;
use syn::{Ident, ItemEnum, ItemStruct, Type};

#[derive(Clone, Debug)]
pub struct Module {
    pub(super) info: ModuleInfo,
    pub(super) scope: ModuleScope,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct ModuleInfo {
    pub(super) visibility: Visibility,
    pub(super) file_path: PathBuf,
    pub(super) module_path: Vec<String>,
    #[derivative(Debug = "ignore")]
    pub(super) source: ModuleSource,
}

/// Mirrors syn::Visibility, but can be created without a token
#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Restricted,
    // Not supported
    Inherited, // Usually means private
}

#[derive(Debug, Clone)]
pub struct Import {
    path: Vec<String>,
    visibility: Visibility,
}

#[derive(Debug, Clone)]
pub enum ModuleSource {
    File(syn::File),
    ModuleInFile(Vec<syn::Item>),
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Struct {
    ident: Ident,
    #[derivative(Debug = "ignore")]
    src: ItemStruct,
    visibility: Visibility,
    path: Vec<String>,
    mirror: bool,
}

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Enum {
    ident: Ident,
    #[derivative(Debug = "ignore")]
    src: ItemEnum,
    visibility: Visibility,
    path: Vec<String>,
    mirror: bool,
}

#[derive(Clone, Debug)]
pub struct TypeAlias {
    ident: String,
    target: Type,
}

#[derive(Debug, Clone)]
pub struct ModuleScope {
    modules: Vec<Module>,
    enums: Vec<Enum>,
    structs: Vec<Struct>,
    imports: Vec<Import>,
    type_alias: Vec<TypeAlias>,
}

impl Module {
    /// Maps out modules, structs and enums within the scope of this module
    pub fn parse(info: ModuleInfo) -> anyhow::Result<Self> {
        let mut scope_modules = Vec::new();
        let mut scope_structs = Vec::new();
        let mut scope_enums = Vec::new();
        let mut scope_types = Vec::new();

        let items = match info.source.as_ref().unwrap() {
            ModuleSource::File(file) => &file.items,
            ModuleSource::ModuleInFile(items) => items,
        };

        for item in items.iter() {
            match item {
                syn::Item::Struct(item_struct) => {
                    let (idents, mirror) = get_ident(&item_struct.ident, &item_struct.attrs);

                    scope_structs.extend(idents.into_iter().map(|ident| {
                        let ident_str = ident.to_string();
                        Struct {
                            ident,
                            src: item_struct.clone(),
                            visibility: syn_vis_to_visibility(&item_struct.vis),
                            path: {
                                let mut path = info.module_path.clone();
                                path.push(ident_str);
                                path
                            },
                            mirror,
                        }
                    }));
                }
                syn::Item::Enum(item_enum) => {
                    let (idents, mirror) = get_ident(&item_enum.ident, &item_enum.attrs);

                    scope_enums.extend(idents.into_iter().map(|ident| {
                        let ident_str = ident.to_string();
                        Enum {
                            ident,
                            src: item_enum.clone(),
                            visibility: syn_vis_to_visibility(&item_enum.vis),
                            path: {
                                let mut path = info.module_path.clone();
                                path.push(ident_str);
                                path
                            },
                            mirror,
                        }
                    }));
                }
                syn::Item::Type(item_type) => {
                    if item_type.generics.where_clause.is_none()
                        && item_type.generics.lt_token.is_none()
                    {
                        scope_types.push(TypeAlias {
                            ident: item_type.ident.to_string(),
                            target: *item_type.ty.clone(),
                        });
                    }
                }
                syn::Item::Mod(item_mod) => {
                    let ident = item_mod.ident.clone();

                    let mut module_path = info.module_path.clone();
                    module_path.push(ident.to_string());

                    scope_modules.push(match &item_mod.content {
                        Some(content) => Module::parse(ModuleInfo {
                            visibility: syn_vis_to_visibility(&item_mod.vis),
                            file_path: info.file_path.clone(),
                            module_path,
                            source: ModuleSource::ModuleInFile(content.1.clone()),
                        })?,
                        None => {
                            let file_path =
                                get_module_file_path(ident.to_string(), &info.file_path);

                            match file_path {
                                Ok(file_path) => {
                                    let source = {
                                        let source_rust_content = read_rust_file(&file_path)?;
                                        debug!("Trying to parse {:?}", file_path);
                                        ModuleSource::File(
                                            syn::parse_file(&source_rust_content).unwrap(),
                                        )
                                    };
                                    Module::parse(ModuleInfo {
                                        visibility: syn_vis_to_visibility(&item_mod.vis),
                                        file_path,
                                        module_path,
                                        source,
                                    })?
                                }
                                Err(tried) => {
                                    warn!(
                                        "Skipping unresolvable module {} (tried {})",
                                        &ident,
                                        tried
                                            .into_iter()
                                            .map(|it| it.to_string_lossy().to_string())
                                            .fold(String::new(), |mut a, b| {
                                                a.push_str(&b);
                                                a.push_str(", ");
                                                a
                                            })
                                    );
                                    continue;
                                }
                            }
                        }
                    });
                }
                _ => {}
            }
        }

        Ok(Module {
            info,
            scope: ModuleScope {
                modules: scope_modules,
                enums: scope_enums,
                structs: scope_structs,
                imports: vec![], // Will be filled in by resolve_imports()
                type_alias: scope_types,
            },
        })
    }

    pub fn collect_structs(&self) -> HashMap<String, &Struct> {
        self.collect_objects(|module| &module.scope.structs, |x| (x.ident.to_string(), x))
    }

    pub fn collect_enums(&self) -> HashMap<String, &Enum> {
        self.collect_objects(|module| &module.scope.enums, |x| (x.ident.to_string(), x))
    }

    pub fn collect_types(&self) -> HashMap<String, Type> {
        self.collect_objects(
            |module| &module.scope.type_alias,
            |x| (x.ident.clone(), x.target.clone()),
        )
    }

    fn collect_objects<'a, T: 'a, F, G, V: 'a>(
        &'a self,
        f: F,
        extract_entry: G,
    ) -> HashMap<String, V>
    where
        F: Fn(&Module) -> &[T],
        G: Fn(&'a T) -> (String, V),
    {
        let mut ans = HashMap::new();
        self.visit_modules(&mut |module| {
            for item in f(module) {
                let (k, v) = extract_entry(item);
                ans.insert(k, v);
            }
        });
        ans
    }

    //noinspection RsNeedlessLifetimes
    fn visit_modules<'a, F: FnMut(&'a Module)>(&'a self, f: &mut F) {
        f(self);
        for scope_module in &self.scope.modules {
            scope_module.visit_modules(f);
        }
    }
}
