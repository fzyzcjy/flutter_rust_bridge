use crate::codegen::preparer::internal_config::PreparerInternalConfig;
use crate::library::commands::ensure_tools_available::ensure_tools_available;

pub(crate) mod internal_config;

pub(super) fn prepare(config: &PreparerInternalConfig) -> anyhow::Result<()> {
    ensure_tools_available(&config.dart_root, config.deps_check, config.needs_ffigen)
}
