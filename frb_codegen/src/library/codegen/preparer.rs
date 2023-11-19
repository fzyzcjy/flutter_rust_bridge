use crate::library::commands::ensure_tools_available::ensure_tools_available;
use std::path::PathBuf;

// TODO unify config
pub(super) struct Config {
    dart_root: PathBuf,
    deps_check: bool,
}

pub(super) fn prepare(config: Config) -> anyhow::Result<()> {
    ensure_tools_available(&config.dart_root, config.deps_check)
}
