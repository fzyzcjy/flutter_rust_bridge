use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PolisherInternalConfig {
    pub duplicated_c_output_path: Vec<PathBuf>,
    pub dart_format_line_length: u32,
    pub add_mod_to_lib: bool,
    pub build_runner: bool,
    pub web_enabled: bool,
    pub dart_root: PathBuf,
    pub rust_crate_dir: PathBuf,
    pub rust_output_path: PathBuf,
    pub c_output_path: Option<PathBuf>,
    pub enable_auto_upgrade: bool,
}
