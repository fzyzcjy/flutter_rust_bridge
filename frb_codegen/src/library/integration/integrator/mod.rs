use crate::utils::path_utils::find_dart_package_dir;
use log::debug;
use std::env;

/// Integrate Rust into existing Flutter project.
pub fn integrate() -> anyhow::Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    todo!("integrate")
}
