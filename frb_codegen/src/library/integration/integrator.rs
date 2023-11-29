use crate::integration::utils::extract_dir_and_modify;
use crate::library::commands::flutter::flutter_pub_add;
use crate::utils::path_utils::find_dart_package_dir;
use anyhow::{Context, Result};
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use log::{debug, warn};
use serde_yaml::Value;
use std::ffi::OsStr;
use std::path::Path;
use std::{env, fs};

static INTEGRATION_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template");

/// Integrate Rust into existing Flutter project.
// ref: https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/
pub fn integrate(enable_integration_test: bool) -> Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    let package_name = get_package_name(&dart_root)?;

    extract_dir_and_modify(
        &INTEGRATION_TEMPLATE_DIR,
        &dart_root,
        &|path, src_raw, existing_content| {
            modify_file(path, src_raw, existing_content, &package_name)
        },
        &|path| filter_file(path, enable_integration_test),
    )?;

    pub_add_dependencies(enable_integration_test)?;

    Ok(())
}

fn modify_file(
    path: &Path,
    src_raw: &[u8],
    existing_content: Option<Vec<u8>>,
    package_name: &str,
) -> Option<Vec<u8>> {
    let src = replace_file_content(src_raw, package_name);

    if let Some(existing_content) = existing_content {
        if path.file_name() == Some(OsStr::new("main.dart")) {
            let existing_content = String::from_utf8(existing_content);
            let commented_existing_content = existing_content
                .map(|x| {
                    format!(
                        "\n\n{}",
                        x.split('\n').map(|line| format!("// {line}")).join("\n")
                    )
                })
                .unwrap_or_default();
            return Some([&src, commented_existing_content.as_bytes()].concat());
        }

        warn!(
            "Skip writing to {path:?} because file already exists. \
            It is suggested to remove that file before running this command to apply the full template."
        );
        return None;
    }

    if path.iter().contains(&OsStr::new("cargokit")) {
        if let Some(comments) = compute_cargokit_comments(path) {
            return Some([comments.as_bytes(), &src].concat());
        }
    }

    Some(src)
}

fn replace_file_content(raw: &[u8], package_name: &str) -> Vec<u8> {
    match String::from_utf8(raw.to_owned()) {
        Ok(raw_str) => raw_str
            .replace("REPLACE_ME_PACKAGE_NAME", package_name)
            .into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

fn filter_file(path: &Path, enable_integration_test: bool) -> bool {
    if path.iter().contains(&OsStr::new("cargokit")) {
        return !vec![".git", ".github", "docs", "test"].contains(&file_name(path));
    }

    if !enable_integration_test && path.iter().contains(&OsStr::new("integration_test")) {
        return false;
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

fn pub_add_dependencies(enable_integration_test: bool) -> Result<()> {
    let mut deps = vec![
        r#"rust_builder:{"path":"./rust_builder"}"#.into(),
        r#"flutter_rust_bridge:{"path":"../../frb_dart"}"#.into(),
        r#"dev:ffigen:^8.0.0"#.into(),
    ];
    if enable_integration_test {
        deps.push(r#"dev:integration_test:{"sdk":"flutter"}"#.into());
    }

    if cfg!(windows) {
        deps = deps.into_iter().map(|x| format!("'{x}'")).collect_vec();
    }

    flutter_pub_add(&deps)
}

fn get_package_name(dart_root: &Path) -> Result<String> {
    let pubspec_yaml: Value = serde_yaml::from_slice(&fs::read(dart_root.join("pubspec.yaml"))?)?;
    Ok(pubspec_yaml
        .get("name")
        .context("no name field")?
        .as_str()
        .context("cannot be str")?
        .to_owned())
}
