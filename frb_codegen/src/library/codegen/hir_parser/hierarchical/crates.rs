use crate::codegen::dumper::Dumper;
use crate::codegen::hir::hierarchical::crates::HirCrate;
use crate::codegen::hir::hierarchical::module::{HirModule, HirVisibility};
use crate::codegen::hir_parser::hierarchical::items::parse_syn_items;
use crate::codegen::hir_parser::hierarchical::module::parse_module;
use crate::codegen::parser::reader::CachedRustReader;
use std::path::Path;

pub(crate) fn parse_crate(rust_crate_dir: &Path, file: syn::File) -> anyhow::Result<HirCrate> {
    let root_modulee = parse_module(&file.items, HirVisibility::Public)?;
    Ok(HirCrate { root_module })
}
