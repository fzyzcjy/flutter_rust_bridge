use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparerInternalConfig {
    pub dart_root: PathBuf,
    pub deps_check: bool,
    pub needs_ffigen: bool,
}
