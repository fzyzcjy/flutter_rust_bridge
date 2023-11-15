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
