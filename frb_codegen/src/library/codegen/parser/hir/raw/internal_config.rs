use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserRawInternalConfig {
    pub rust_crate_dir: PathBuf,
}
