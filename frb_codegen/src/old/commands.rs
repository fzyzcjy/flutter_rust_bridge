use crate::Result;
use cargo_metadata::VersionReq;
use lazy_static::lazy_static;
use log::warn;
use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Mutex;

use crate::command_run;
use crate::utils::command_runner::{call_shell, execute_command};
use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use log::{debug, info};

mod error;
pub use error::Error;

lazy_static! {
    pub(crate) static ref FFI_REQUIREMENT: VersionReq =
        VersionReq::parse(">= 2.0.1, < 3.0.0").unwrap();
    pub(crate) static ref FFIGEN_REQUIREMENT: VersionReq =
        VersionReq::parse(">= 8.0.0, < 10.0.0").unwrap();
}

pub fn ensure_tools_available(dart_root: &Path, skip_deps_check: bool) -> Result<(), Error> {
    let repo = DartRepository::from_str(dart_root)?;
    if !repo.toolchain_available() {
        Err(Error::MissingExe(repo.toolchain.to_string()))?;
    }

    if !skip_deps_check {
        repo.has_specified("ffi", DartDependencyMode::Main, &FFI_REQUIREMENT)?;
        repo.has_installed("ffi", DartDependencyMode::Main, &FFI_REQUIREMENT)?;

        repo.has_specified("ffigen", DartDependencyMode::Dev, &FFIGEN_REQUIREMENT)?;
        repo.has_installed("ffigen", DartDependencyMode::Dev, &FFIGEN_REQUIREMENT)?;
    }

    Ok(())
}
