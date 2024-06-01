use std::path::PathBuf;
use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserReaderInternalConfig {
    pub rust_crate_dir: PathBuf,
}
