use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir, DirEntry};
use log::debug;
use std::path::{Path, PathBuf};
use std::{env, fs};

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

// ref: `Dir::extract`
fn extract_dir_and_modify(d: &Dir, base_path: &Path) -> Result<()> {
    for entry in d.entries() {
        let path = base_path.join(entry.path());
        match entry {
            DirEntry::Dir(d) => {
                fs::create_dir_all(&path)?;
                d.extract(base_path)?;
            }
            DirEntry::File(f) => {
                fs::write(path, f.contents())?;
            }
        }
    }
    Ok(())
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
