use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir};
use log::debug;
use std::env;
use std::path::{Path, PathBuf};

static INTEGRATION_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template");

/// Integrate Rust into existing Flutter project.
pub fn integrate() -> Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    handle_cargokit_dir(&dart_root)?;
    handle_rust_dir(&dart_root)?;

    handle_ios_or_macos(&dart_root, "ios")?;
    handle_ios_or_macos(&dart_root, "macos")?;
    handle_windows_or_linux(&dart_root, "windows")?;
    handle_windows_or_linux(&dart_root, "linux")?;
    handle_android(&dart_root)?;

    Ok(())
}

fn handle_cargokit_dir(dart_root: &Path) -> Result<()> {
    let dir_cargokit = dart_root.join("cargokit");
    copy_cargokit_dir(dart_root, &dir_cargokit)?;
    cargokit_add_prelude(dart_root, &dir_cargokit)
}

fn copy_cargokit_dir(dart_root: &Path, dir_cargokit: &Path) -> Result<()> {
    Ok(INTEGRATION_TEMPLATE_DIR
        .get_dir("cargokit")
        .unwrap()
        .extract(dir_cargokit)?)
}

fn cargokit_add_prelude(dart_root: &Path, dir_cargokit: &Path) -> Result<()> {
    todo!()
}

fn handle_rust_dir(dart_root: &Path) -> Result<()> {
    // TODO the "cdylib + staticlib"
    todo!()
}

fn handle_ios_or_macos(dart_root: &Path, dir_name: &str) -> Result<()> {
    todo!()
}

fn handle_windows_or_linux(dart_root: &Path, dir_name: &str) -> Result<()> {
    todo!()
}

fn handle_android(dart_root: &Path) -> Result<()> {
    todo!()
}
