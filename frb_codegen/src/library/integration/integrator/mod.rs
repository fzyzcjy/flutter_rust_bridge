use crate::integration::integrator::utils::extract_dir_and_modify;
use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir, DirEntry};
use itertools::Itertools;
use log::debug;
use std::path::{Path, PathBuf};
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
        &dart_root,
        &|p, raw| {
            if vec![".gitignore"].contains(&file_name(p)) {
                return raw.to_owned();
            }

            let comment_leading = match file_extension(p) {
                "dart" | "md" | "gradle" | "" => "///",
                "yaml" | "toml" | "sh" => "#",
                "lock" | "cmake" | "ps1" | ".cmd" => return raw.to_owned(),
                ext => unreachable!("unexpected file extension for p={:?} ext={}", p, ext),
            };

            let comments = (CARGOKIT_PRELUDE.iter())
                .map(|line| format!("{comment_leading} {line}"))
                .join("\n");

            [&comments.as_bytes()[..], &raw[..]].concat()
        },
        &|p| !vec![".git", ".github", "docs", "test"].contains(&file_name(p)),
    )
}

fn file_name(p: &Path) -> &str {
    &p.file_name().unwrap().to_str().unwrap()
}

fn file_extension(p: &Path) -> &str {
    &p.extension().unwrap_or_default().to_str().unwrap()
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
        &dart_root,
        &|_, raw| raw.to_owned(),
        &|p| true,
    )
}

fn handle_ios_or_macos(dart_root: &Path, dir_name: &str) -> Result<()> {
    // TODO
    Ok(())
}

fn handle_windows_or_linux(dart_root: &Path, dir_name: &str) -> Result<()> {
    // TODO
    // let path = dart_root.join(dir_name).join("CMakeLists.txt");
    Ok(())
}

fn handle_android(dart_root: &Path) -> Result<()> {
    let path = dart_root.join("android").join("build.gradle");
    let text = format!(
        r#"
// flutter_rust_bridge + cargokit BEGIN
apply from: "../cargokit/gradle/plugin.gradle"
cargokit {{
    manifestDir = "../rust"
    libname = "rust_lib"
}}
// flutter_rust_bridge + cargokit END
"#
    );
    fs::write(&path, fs::read_to_string(&path)? + &text)?;
    Ok(())
}
