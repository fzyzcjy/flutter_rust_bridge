use crate::codegen::parser::reader::read_rust_file;
use crate::codegen::parser::source_graph::modules::{
    Enum, Module, ModuleInfo, ModuleScope, ModuleSource, Struct, TypeAlias,
};
use log::{debug, warn};

impl Module {
    /// Maps out modules, structs and enums within the scope of this module
    pub fn parse(info: ModuleInfo) -> anyhow::Result<Self> {
        let mut scope_modules = Vec::new();
        let mut scope_structs = Vec::new();
        let mut scope_enums = Vec::new();
        let mut scope_types = Vec::new();

        let items = match &info.source {
            ModuleSource::File(file) => &file.items,
            ModuleSource::ModuleInFile(items) => &items,
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
                            visibility: item_struct.vis.into(),
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
                            visibility: item_enum.vis.into(),
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
                            visibility: item_mod.vis.into(),
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
                                        visibility: item_mod.vis.into(),
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
}
