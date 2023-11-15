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
