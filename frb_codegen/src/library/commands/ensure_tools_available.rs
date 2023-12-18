use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use crate::utils::path_utils::path_to_string;
use anyhow::bail;
use cargo_metadata::VersionReq;
use lazy_static::lazy_static;
use std::path::Path;
use std::str::FromStr;

lazy_static! {
    pub(crate) static ref FFIGEN_REQUIREMENT: VersionReq =
        VersionReq::parse(">= 8.0.0, < 10.0.0").unwrap();
}

pub fn ensure_tools_available(dart_root: &Path, enable_deps_check: bool) -> anyhow::Result<()> {
    let repo = DartRepository::from_str(&path_to_string(dart_root)?)?;
    if !repo.toolchain_available() {
        bail!("Dart/Flutter toolchain not available");
    }

    if enable_deps_check {
        repo.has_specified_and_installed("ffigen", DartDependencyMode::Dev, &FFIGEN_REQUIREMENT)?;
    }

    Ok(())
}
