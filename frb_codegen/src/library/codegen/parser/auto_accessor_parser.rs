use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::parser::misc::extract_src_types_in_paths;
use crate::codegen::parser::source_graph::modules::Struct;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) fn parse_auto_accessors(
    src_structs: &HashMap<String, &Struct>,
    rust_input_paths: &[PathBuf],
    rust_crate_dir: &Path,
) -> anyhow::Result<Vec<IrFunc>> {
    let src_structs_in_paths =
        extract_src_types_in_paths(src_structs, rust_input_paths, rust_crate_dir)?;
    todo!()
}
