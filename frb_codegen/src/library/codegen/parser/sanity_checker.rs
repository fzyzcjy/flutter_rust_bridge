use crate::codegen::dumper::Dumper;
use crate::codegen::parser::reader::CachedRustReader;
use std::path::{Path, PathBuf};

pub(crate) fn check_suppressed_input_path_no_content(
    rust_suppressed_input_paths: &[PathBuf],
    rust_crate_dir: &Path,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
) {
    TODO
}
