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

fn handle_rust_dir(dart_root: &Path) -> Result<()> {
    fs::create_dir_all(dart_root.join("rust"))?;
    extract_dir_and_modify(
        INTEGRATION_TEMPLATE_DIR.get_dir("rust").unwrap(),
        dart_root,
        &|_, raw| raw.to_owned(),
        &|_p| true,
    )
}

fn handle_ios_or_macos(_dart_root: &Path, _dir_name: &str) -> Result<()> {
    // TODO
    Ok(())
}

fn handle_windows_or_linux(_dart_root: &Path, _dir_name: &str) -> Result<()> {
    // TODO
    // let path = dart_root.join(dir_name).join("CMakeLists.txt");
    Ok(())
}

fn handle_android(dart_root: &Path) -> Result<()> {
    let path = dart_root.join("android").join("build.gradle");
    let text = format!(
        r#"
// {BEGIN_COMMENT}
apply from: "../cargokit/gradle/plugin.gradle"
cargokit {{
    manifestDir = "../rust"
    libname = "rust_lib"
}}
// {END_COMMENT}
"#
    );
    fs::write(&path, fs::read_to_string(&path)? + &text)?;
    Ok(())
}

const BEGIN_COMMENT: &str = "BEGIN_FLUTTER_RUST_BRIDGE_WITH_CARGOKIT";
const END_COMMENT: &str = "END_FLUTTER_RUST_BRIDGE_WITH_CARGOKIT";
