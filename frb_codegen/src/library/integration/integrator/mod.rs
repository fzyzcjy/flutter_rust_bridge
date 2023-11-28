use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir};
use log::debug;
use std::env;

static INTEGRATION_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template");

/// Integrate Rust into existing Flutter project.
pub fn integrate() -> Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    handle_cargokit_dir()?;
    handle_rust_dir()?;

    handle_ios_or_macos("ios")?;
    handle_ios_or_macos("macos")?;
    handle_windows_or_linux("windows")?;
    handle_windows_or_linux("linux")?;
    handle_android()?;

    Ok(())
}

fn handle_cargokit_dir() -> Result<()> {
    copy_cargokit_dir()?;
    cargokit_add_prelude()
}

fn copy_cargokit_dir() -> Result<()> {
    todo!()
}

fn cargokit_add_prelude() -> Result<()> {
    todo!()
}

fn handle_rust_dir() -> Result<()> {
    // TODO the "cdylib + staticlib"
    todo!()
}

fn handle_ios_or_macos(dir_name: &str) -> Result<()> {
    todo!()
}

fn handle_windows_or_linux(dir_name: &str) -> Result<()> {
    todo!()
}

fn handle_android() -> Result<()> {
    todo!()
}
