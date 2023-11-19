use crate::library::commands::ensure_tools_available::ensure_tools_available;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparerInternalConfig {
    pub dart_root: PathBuf,
    pub deps_check: bool,
}

pub(super) fn prepare(config: PreparerInternalConfig) -> anyhow::Result<()> {
    ensure_tools_available(&config.dart_root, config.deps_check)
}
