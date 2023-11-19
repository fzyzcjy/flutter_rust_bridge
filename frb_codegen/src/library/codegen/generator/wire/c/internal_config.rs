use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireCInternalConfig {
    pub(crate) rust_crate_dir: PathBuf,
    pub(crate) rust_output_path: PathBuf,
    pub(crate) c_output_path: PathBuf,
    // TODO originally from: `generated_rust.extern_func_names`
    pub(crate) extern_func_names: Vec<String>,
    // TODO originally created via `get_c_struct_names`, should calc it from wire-rust layer, in analogy to `extern_func_names`
    pub(crate) extern_struct_names: Vec<String>,
}
