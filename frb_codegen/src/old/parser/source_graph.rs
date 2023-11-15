/*
    Things this doesn't currently support that it might need to later:

    - Import parsing is unfinished and so is currently disabled
    - When import parsing is enabled:
        - Import renames (use a::b as c) - these are silently ignored
        - Imports that start with two colons (use ::a::b) - these are also silently ignored
*/

use std::{
    collections::HashMap,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

use cargo_metadata::MetadataCommand;
use log::{debug, warn};
use syn::{Attribute, Ident, ItemEnum, ItemStruct, PathArguments, Type, UseTree};

use super::ParserResult;
use crate::{parser::markers, utils::misc::read_rust_file};

fn syn_vis_to_visibility(vis: &syn::Visibility) -> Visibility {
    match vis {
        syn::Visibility::Public(_) => Visibility::Public,
        syn::Visibility::Restricted(_) => Visibility::Restricted,
        syn::Visibility::Inherited => Visibility::Inherited,
    }
}

/// Get a struct or enum ident, possibly remapped by a mirror marker
fn get_ident(ident: &Ident, attrs: &[Attribute]) -> (Vec<Ident>, bool) {
    // TODO use `metadata.mirror()`
    let res = markers::extract_mirror_marker(attrs)
        .into_iter()
        .filter_map(|path| {
            // eq: path.get_ident().map(Clone::clone)
            if path.leading_colon.is_none()
                && path.segments.len() == 1
                && path.segments[0].arguments == PathArguments::None
            {
                Some(path.segments.into_iter().next().unwrap().ident)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mirror = !res.is_empty();
    if mirror {
        (res, mirror)
    } else {
        (vec![ident.clone()], mirror)
    }
}

fn try_get_module_file_path(
    folder_path: &Path,
    module_name: &str,
    tried: &mut Vec<PathBuf>,
) -> Option<PathBuf> {
    let file_path = folder_path.join(module_name).with_extension("rs");
    if file_path.exists() {
        return Some(file_path);
    }
    tried.push(file_path);

    let file_path = folder_path.join(module_name).join("mod.rs");
    if file_path.exists() {
        return Some(file_path);
    }
    tried.push(file_path);

    None
}

fn get_module_file_path(
    module_name: String,
    parent_module_file_path: &Path,
) -> ParserResult<PathBuf, Vec<PathBuf>> {
    let mut tried = Vec::new();

    if let Some(file_path) = try_get_module_file_path(
        parent_module_file_path.parent().unwrap(),
        &module_name,
        &mut tried,
    ) {
        return Ok(file_path);
    }
    if let Some(file_path) = try_get_module_file_path(
        &parent_module_file_path.with_extension(""),
        &module_name,
        &mut tried,
    ) {
        return Ok(file_path);
    }
    Err(tried)
}
