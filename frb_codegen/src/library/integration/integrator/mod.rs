use crate::utils::path_utils::find_dart_package_dir;
use include_dir::{include_dir, Dir};
use log::debug;
use std::env;

static INTEGRATION_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template");

/// Integrate Rust into existing Flutter project.
pub fn integrate() -> anyhow::Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    todo!("integrate")
}
