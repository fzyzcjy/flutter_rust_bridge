use crate::codegen::dumper::Dumper;
use crate::codegen::hir::hierarchical::crates::HirCrate;
use crate::codegen::hir::hierarchical::module::HirModule;
use crate::codegen::hir_parser::hierarchical::items::parse_syn_items;
use crate::codegen::hir_parser::hierarchical::module::parse_module;
use crate::codegen::parser::reader::CachedRustReader;
use std::path::Path;

pub(crate) fn parse_crate(
    rust_crate_dir: &Path,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<HirCrate> {
    let file = cached_rust_reader.read_rust_crate(rust_crate_dir, dumper)?;
    let root_modulee = parse_syn_items(&file.items)?;
    Ok(HirCrate { root_module })
}
