use crate::codegen::dumper::Dumper;
use crate::codegen::hir::hierarchical::crates::HirCrate;
use crate::codegen::parser::reader::CachedRustReader;
use std::path::Path;

pub(crate) fn parse_crate(
    rust_crate_dir: &Path,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) -> anyhow::Result<HirCrate> {
    let crate_ast = cached_rust_reader.read_rust_crate(rust_crate_dir, dumper)?;
    todo!()
}
