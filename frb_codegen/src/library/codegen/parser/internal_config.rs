use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::codegen::config::internal_config::Namespace;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserInternalConfig {
    pub rust_input_path_pack: RustInputPathPack,
    pub rust_crate_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RustInputPathPack {
    pub rust_input_path: HashMap<Namespace, PathBuf>,
}
