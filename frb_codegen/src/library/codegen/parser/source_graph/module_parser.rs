use crate::codegen::dumper::Dumper;
use crate::codegen::hir::hierarchical::module::Module;
use crate::codegen::hir::hierarchical::module::ModuleInfo;
use crate::codegen::hir::hierarchical::module::ModuleScope;
use crate::codegen::hir::hierarchical::module::ModuleSource;
use crate::codegen::hir::hierarchical::module::Visibility;
use crate::codegen::hir::hierarchical::struct_or_enum::Enum;
use crate::codegen::hir::hierarchical::struct_or_enum::StructOrEnum;
use crate::codegen::hir::hierarchical::type_alias::TypeAlias;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::reader::CachedRustReader;
use crate::codegen::parser::Struct;
use crate::utils::path_utils::{find_rust_crate_dir, path_to_string};
use anyhow::Context;
use itertools::Itertools;
use log::{debug, warn};
use std::path::{Path, PathBuf};
use syn::token::Brace;
use syn::{Attribute, Ident, Item, ItemEnum, ItemMod, ItemStruct, ItemType, PathArguments};

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
