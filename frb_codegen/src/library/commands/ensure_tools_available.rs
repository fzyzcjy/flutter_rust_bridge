use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use crate::utils::path_utils::path_to_string;
use anyhow::bail;
use cargo_metadata::VersionReq;
use lazy_static::lazy_static;
use std::path::Path;
use std::str::FromStr;

lazy_static! {
    pub(crate) static ref FFIGEN_REQUIREMENT: VersionReq = VersionReq::parse(">= 8.0.0").unwrap();
}

pub fn ensure_tools_available(
    dart_root: &Path,
    enable_deps_check: bool,
    needs_ffigen: bool,
) -> anyhow::Result<()> {
    let repo = DartRepository::from_str(&path_to_string(dart_root)?)?;
    if !repo.toolchain_available() {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("Dart/Flutter toolchain not available");
        // frb-coverage:ignore-end
    }

    if enable_deps_check && needs_ffigen {
        repo.has_specified_and_installed("ffigen", DartDependencyMode::Dev, &FFIGEN_REQUIREMENT)?;
        // This empty bracket ("}") is weirdly not covered, while lines above and below it are
        // frb-coverage:ignore-start
    }
    // frb-coverage:ignore-end

    Ok(())
}
