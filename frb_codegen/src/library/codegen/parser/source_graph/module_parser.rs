use crate::codegen::dumper::Dumper;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::reader::CachedRustReader;
use crate::codegen::parser::source_graph::modules::{
    Enum, Module, ModuleInfo, ModuleScope, ModuleSource, Struct, StructOrEnum, TypeAlias,
    Visibility,
};
use crate::utils::path_utils::{find_rust_crate_dir, path_to_string};
use anyhow::Context;
use itertools::Itertools;
use log::{debug, warn};
use std::path::{Path, PathBuf};
use syn::token::Brace;
use syn::{Attribute, Ident, Item, ItemEnum, ItemMod, ItemStruct, ItemType, PathArguments};

impl Module {
    /// Maps out modules, structs and enums within the scope of this module
    //
    // Things this doesn't currently support that it might need to later:
    //
    // - Import parsing is unfinished and so is currently disabled
    // - When import parsing is enabled:
    //     - Import renames (use a::b as c) - these are silently ignored
    //     - Imports that start with two colons (use ::a::b) - these are also silently ignored

    pub(crate) fn parse(
        info: ModuleInfo,
        cached_rust_reader: &mut CachedRustReader,
        dumper: &Dumper,
    ) -> anyhow::Result<Self> {
        debug!("parse START info={info:?}");

        let mut scope_modules = Vec::new();
        let mut scope_structs = Vec::new();
        let mut scope_enums = Vec::new();
        let mut scope_types = Vec::new();

        let items = match &info.source {
            ModuleSource::File(file) => &file.items,
            ModuleSource::ModuleInFile(items) => items,
        };

        for item in items.iter() {
            match item {
                syn::Item::Struct(item_struct) => {
                    scope_structs.extend(parse_syn_item_struct(&info, item_struct)?);
                }
                syn::Item::Enum(item_enum) => {
                    scope_enums.extend(parse_syn_item_enum(&info, item_enum)?);
                }
                syn::Item::Type(item_type) => {
                    scope_types.extend(parse_syn_item_type(item_type));
                }
                syn::Item::Mod(item_mod) => {
                    scope_modules.extend(parse_syn_item_mod(
                        &info,
                        item_mod,
                        cached_rust_reader,
                        dumper,
                    )?);
                }
                _ => {}
            }
        }

        let ans = Module {
            info: info.clone(),
            scope: ModuleScope {
                modules: scope_modules,
                enums: scope_enums,
                structs: scope_structs,
                // imports: vec![], // Will be filled in by resolve_imports()
                type_alias: scope_types,
            },
        };

        debug!("parse END info={info:?}");
        Ok(ans)
    }
}

fn parse_syn_item_struct(info: &ModuleInfo, item: &ItemStruct) -> anyhow::Result<Vec<Struct>> {
    parse_syn_item_struct_or_enum(info, item, &item.ident, &item.attrs, &item.vis, Struct)
}

fn parse_syn_item_enum(info: &ModuleInfo, item: &ItemEnum) -> anyhow::Result<Vec<Enum>> {
    parse_syn_item_struct_or_enum(info, item, &item.ident, &item.attrs, &item.vis, Enum)
}

fn parse_syn_item_struct_or_enum<I: Clone, F, T>(
    info: &ModuleInfo,
    item: &I,
    item_ident: &Ident,
    item_attrs: &[Attribute],
    item_vis: &syn::Visibility,
    constructor: F,
) -> anyhow::Result<Vec<T>>
where
    F: Fn(StructOrEnum<I>) -> T,
{
    debug!("parse_syn_item_struct_or_enum item_ident={item_ident:?}");

    let ParseMirrorIdentOutput { idents, mirror } = parse_mirror_ident(item_ident, item_attrs)?;
    Ok(idents
        .into_iter()
        .map(|ident| {
            let ident_str = ident.to_string();
            constructor(StructOrEnum {
                ident,
                src: item.clone(),
                visibility: Visibility::from_syn(item_vis),
                path: {
                    let mut path = info.module_path.clone();
                    path.push(ident_str);
                    path
                },
                mirror,
            })
        })
        .collect_vec())
}

fn parse_syn_item_type(item_type: &ItemType) -> Option<TypeAlias> {
    debug!("parse_syn_item_struct_or_enum item_type={item_type:?}");

    if item_type.generics.where_clause.is_none() && item_type.generics.lt_token.is_none() {
        Some(TypeAlias {
            ident: item_type.ident.to_string(),
            target: *item_type.ty.clone(),
        })
    } else {
        None
    }
}

fn parse_syn_item_mod(
    info: &ModuleInfo,
    item_mod: &ItemMod,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<Option<Module>> {
    let ident = item_mod.ident.clone();

    let module_path = {
        let mut x = info.module_path.clone();
        x.push(ident.to_string());
        x
    };

    debug!("parse_syn_item_mod module_path={module_path:?}");

    Ok(match &item_mod.content {
        Some(content) => parse_syn_item_mod_contentful(
            info,
            item_mod,
            module_path,
            content,
            cached_rust_reader,
            dumper,
        )?,
        None => parse_syn_item_mod_contentless(
            info,
            item_mod,
            module_path,
            ident,
            cached_rust_reader,
            dumper,
        )?,
    })
}

fn parse_syn_item_mod_contentful(
    info: &ModuleInfo,
    item_mod: &ItemMod,
    module_path: Vec<String>,
    content: &(Brace, Vec<Item>),
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<Option<Module>> {
    debug!("parse_syn_item_mod_contentful module_path={module_path:?}");

    Ok(Some(Module::parse(
        ModuleInfo {
            visibility: Visibility::from_syn(&item_mod.vis),
            file_path: info.file_path.clone(),
            module_path,
            source: ModuleSource::ModuleInFile(content.1.clone()),
        },
        cached_rust_reader,
        dumper,
    )?))
}

fn parse_syn_item_mod_contentless(
    info: &ModuleInfo,
    item_mod: &ItemMod,
    module_path: Vec<String>,
    ident: Ident,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<Option<Module>> {
    debug!("parse_syn_item_mod_contentless module_path={module_path:?}");

    let file_path_candidates = get_module_file_path_candidates(ident.to_string(), &info.file_path);
    debug!(
        "file_path_candidates {:?} {:?} {:?}",
        ident.to_string(),
        &info.file_path,
        &file_path_candidates
    );

    if let Some(file_path) = first_existing_path(&file_path_candidates) {
        let rust_crate_dir_for_file = find_rust_crate_dir(file_path)?;
        let source_rust_content =
            cached_rust_reader.read_rust_file(file_path, &rust_crate_dir_for_file, dumper)?;
        debug!("Trying to parse {:?}", file_path);
        let source = ModuleSource::File(syn::parse_file(&source_rust_content).unwrap());

        Ok(Some(Module::parse(
            ModuleInfo {
                visibility: Visibility::from_syn(&item_mod.vis),
                file_path: file_path.to_owned(),
                module_path,
                source,
            },
            cached_rust_reader,
            dumper,
        )?))
    } else {
        // We do not care about the warning
        // frb-coverage:ignore-start
        warn!(
            "Skipping unresolvable module {} (tried {})",
            &ident,
            file_path_candidates
                .iter()
                .map(|p| path_to_string(p))
                .collect::<anyhow::Result<Vec<_>>>()?
                .join(", ")
        );
        Ok(None)
        // frb-coverage:ignore-end
    }
}

fn get_module_file_path_candidates(
    module_name: String,
    parent_module_file_path: &Path,
) -> Vec<PathBuf> {
    [
        parent_module_file_path.parent().unwrap().to_owned(),
        parent_module_file_path.with_extension(""),
    ]
    .iter()
    .flat_map(|folder_path| {
        [
            folder_path.join(&module_name).with_extension("rs"),
            folder_path.join(&module_name).join("mod.rs"),
        ]
    })
    .collect_vec()
}

fn first_existing_path(path_candidates: &[PathBuf]) -> Option<&PathBuf> {
    path_candidates.iter().find(|path| path.exists())
}

struct ParseMirrorIdentOutput {
    idents: Vec<Ident>,
    mirror: bool,
}

/// Get a struct or enum ident, possibly remapped by a mirror marker
fn parse_mirror_ident(
    ident: &Ident,
    attrs: &[Attribute],
) -> anyhow::Result<ParseMirrorIdentOutput> {
    let attributes = FrbAttributes::parse(attrs)
        .with_context(|| format!("when parsing ident={ident:?} attrs={attrs:?}"))?;
    let mirror_info = attributes.mirror();

    let res = mirror_info
        .into_iter()
        .filter_map(|path| {
            if path.leading_colon.is_none()
                && path.segments.len() == 1
                && path.segments[0].arguments == PathArguments::None
            {
                Some(path.segments.into_iter().next().unwrap().ident)
            } else {
                None
            }
        })
        .collect_vec();

    let mirror = !res.is_empty();

    Ok(ParseMirrorIdentOutput {
        idents: if mirror { res } else { vec![ident.clone()] },
        mirror,
    })
}

impl Visibility {
    fn from_syn(value: &syn::Visibility) -> Self {
        match value {
            syn::Visibility::Public(_) => Visibility::Public,
            syn::Visibility::Restricted(_) => Visibility::Restricted,
            syn::Visibility::Inherited => Visibility::Inherited,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::parser::source_graph::module_parser::get_module_file_path_candidates;
    use std::path::PathBuf;

    #[test]
    fn test_get_module_file_path_candidates_simple() {
        let actual =
            get_module_file_path_candidates("api".to_owned(), &PathBuf::from("/hello/src/main.rs"));
        let expect = vec![
            PathBuf::from("/hello/src/api.rs"),
            PathBuf::from("/hello/src/api/mod.rs"),
            PathBuf::from("/hello/src/main/api.rs"),
            PathBuf::from("/hello/src/main/api/mod.rs"),
        ];
        assert_eq!(actual, expect);
    }
}
