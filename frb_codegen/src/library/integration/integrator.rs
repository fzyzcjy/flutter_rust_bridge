use crate::integration::utils::extract_dir_and_modify;
use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use log::debug;
use std::path::Path;
use std::{env, fs};

static INTEGRATION_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template");

/// Integrate Rust into existing Flutter project.
// ref: https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/
pub fn integrate() -> Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    extract_dir_and_modify(
        &INTEGRATION_TEMPLATE_DIR,
        &dart_root,
        &modify_file,
        &filter_file,
    )?;

    Ok(())
}

fn modify_file(path: &Path, raw: &[u8]) -> Vec<u8> {
    if path.iter().contains("cargokit".into()) {
        if let Some(comments) = compute_cargokit_comments(path) {
            return [comments.as_bytes(), raw].concat();
        }
    }
    raw.to_owned()
}

fn filter_file(path: &Path) -> bool {
    if path.iter().contains("cargokit".into()) {
        return !vec![".git", ".github", "docs", "test"].contains(&file_name(path));
    }
    true
}

fn compute_cargokit_comments(path: &Path) -> Option<String> {
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
