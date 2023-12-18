//! Build web platform for a Flutter+Rust app

use crate::utils::dart_repository::dart_repo::DartRepository;
use crate::utils::path_utils::{find_dart_package_dir, path_to_string};
use anyhow::{bail, Context};
use log::debug;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::{env, fs};

// We make the core build-web logic in Dart, and Rust is just a wrapper.
// This is because, in the future, the build-web logic may be packaged with user libraries
// and invoked in machines without flutter_rust_bridge_codegen binary.
pub fn build(dart_root: Option<PathBuf>, args: Vec<String>) -> anyhow::Result<()> {
    let dart_root = parse_dart_root(dart_root)?;
    debug!("build dart_root={dart_root:?} args={args:?}");
    execute_dart_command(&dart_root, &args)
}

fn parse_dart_root(dart_root: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    dart_root
        .map(|x| Ok(fs::canonicalize(x)?))
        .unwrap_or_else(|| {
            find_dart_package_dir(&env::current_dir()?)
                .context("Please provide --dart-root, or run command inside a Flutter/Dart package")
        })
}

fn execute_dart_command(dart_root: &Path, args: &[String]) -> anyhow::Result<()> {
    let repo = DartRepository::from_str(&path_to_string(dart_root)?)?;

    let status = Command::new("dart")
        .current_dir(dart_root)
        .args(repo.command_extra_args())
        .arg("run")
        .arg("flutter_rust_bridge")
        .arg("build-web")
        .arg("--dart-root")
        .arg(dart_root)
        .args(args)
        .status()?;

    if !status.success() {
        bail!("Fail to execute command, please see logs above for details.")
    }

    Ok(())
}
