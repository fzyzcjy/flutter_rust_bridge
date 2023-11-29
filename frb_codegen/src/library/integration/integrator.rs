use crate::integration::integrator::utils::extract_dir_and_modify;
use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use log::debug;
use std::path::Path;
use std::{env, fs};

mod utils;

static INTEGRATION_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template");

/// Integrate Rust into existing Flutter project.
// ref: https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/
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
    fs::create_dir_all(dart_root.join("cargokit"))?;
    extract_dir_and_modify(
        INTEGRATION_TEMPLATE_DIR.get_dir("cargokit").unwrap(),
        dart_root,
        &|path, raw| {
            if let Some(comments) = compute_comments(path) {
                [comments.as_bytes(), raw].concat()
            } else {
                raw.to_owned()
            }
        },
        &|p| !vec![".git", ".github", "docs", "test"].contains(&file_name(p)),
    )
}

fn compute_comments(path: &Path) -> Option<String> {
    if vec![".gitignore"].contains(&file_name(path)) {
        return None;
    }

    let comment_leading = match file_extension(path) {
        "dart" | "md" | "gradle" | "" => "///",
        "yaml" | "toml" | "sh" => "#",
        "lock" | "cmake" | "ps1" | "cmd" => return None,
        ext => unreachable!("unexpected file extension for path={:?} ext={}", path, ext),
    };

    Some(
        (CARGOKIT_PRELUDE.iter())
            .map(|line| format!("{comment_leading} {line}"))
            .join("\n"),
    )
}

fn file_name(p: &Path) -> &str {
    p.file_name().unwrap().to_str().unwrap()
}

fn file_extension(p: &Path) -> &str {
    p.extension().unwrap_or_default().to_str().unwrap()
}

const CARGOKIT_PRELUDE: &[&str] = &[
    "This is copied from cargokit, TODO",
    "TODO explain more",
    "\n\n",
];
