use crate::integration::utils::extract_dir_and_modify;
use crate::library::commands::flutter::flutter_pub_add;
use crate::library::commands::format_dart::format_dart;
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
pub fn integrate(enable_integration_test: bool, enable_local_dependency: bool) -> Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    let package_name = get_package_name(&dart_root)?;

    extract_dir_and_modify(
        &INTEGRATION_TEMPLATE_DIR,
        &dart_root,
        &|path, src_raw, existing_content| {
            modify_file(
                path,
                src_raw,
                existing_content,
                &package_name,
                enable_local_dependency,
            )
        },
        &|path| filter_file(path, enable_integration_test),
    )?;

    modify_permissions(&dart_root)?;

    pub_add_dependencies(enable_integration_test, enable_local_dependency)?;

    format_dart(&[dart_root], 80)?;

    Ok(())
}

fn modify_permissions(dart_root: &Path) -> Result<()> {
    let dir_cargokit = dart_root.join("rust_builder").join("cargokit");

    #[cfg(not(windows))]
    {
        set_permission_executable(&dir_cargokit.join("build_pod.sh"))?;
        set_permission_executable(&dir_cargokit.join("run_build_tool.sh"))?;
        set_permission_executable(&dir_cargokit.join("run_build_tool.cmd"))?;
    }

    Ok(())
}

#[cfg(not(windows))]
fn set_permission_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)?;
    Ok(())
}

fn modify_file(
    path: &Path,
    src_raw: &[u8],
    existing_content: Option<Vec<u8>>,
    package_name: &str,
    enable_local_dependency: bool,
) -> Option<Vec<u8>> {
    let src = replace_file_content(src_raw, package_name, enable_local_dependency);

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

fn replace_file_content(raw: &[u8], package_name: &str, enable_local_dependency: bool) -> Vec<u8> {
    match String::from_utf8(raw.to_owned()) {
        Ok(raw_str) => raw_str
            .replace("REPLACE_ME_PACKAGE_NAME", package_name)
            .replace(
                "REPLACE_ME_RUST_FRB_DEPENDENCY",
                &if enable_local_dependency {
                    r#"{ path = "../../../frb_rust" }"#.to_owned()
                } else {
                    format!(r#""={}""#, env!("CARGO_PKG_VERSION"))
                },
            )
            .into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

fn filter_file(path: &Path, enable_integration_test: bool) -> bool {
    if path.iter().contains(&OsStr::new("cargokit")) {
        return ![".git", ".github", "docs", "test"].contains(&file_name(path));
    }

    if !enable_integration_test && path.iter().contains(&OsStr::new("integration_test")) {
        return false;
    }

    true
}

fn compute_cargokit_comments(path: &Path) -> Option<String> {
    if [".gitignore"].contains(&file_name(path)) {
        return None;
    }

    let comment_leading = match file_extension(path) {
        "dart" | "md" | "gradle" | "" => "///",
        "yaml" | "toml" => "#",
        // Do not add prelude for `sh`, since it can contain things like `#!/bin/bash`
        // which must be at first line
        "lock" | "cmake" | "sh" | "ps1" | "cmd" => return None,
        ext => unreachable!("unexpected file extension for path={:?} ext={}", path, ext),
    };

    Some(
        (CARGOKIT_PRELUDE.iter())
            .map(|line| format!("{comment_leading} {line}"))
            .join("\n")
            + "\n\n",
    )
}

fn file_name(p: &Path) -> &str {
    p.file_name().unwrap().to_str().unwrap()
}

fn file_extension(p: &Path) -> &str {
    p.extension().unwrap_or_default().to_str().unwrap()
}

const CARGOKIT_PRELUDE: &[&str] = &[
    "This is copied from Cargokit (which is the official way to use it currently)", //
    "Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin",
];

fn pub_add_dependencies(
    enable_integration_test: bool,
    enable_local_dependency: bool,
) -> Result<()> {
    flutter_pub_add(&["rust_builder".into(), "--path=rust_builder".into()])?;

    flutter_pub_add(&if enable_local_dependency {
        vec![
            "flutter_rust_bridge".to_owned(),
            "--path=../../frb_dart".to_owned(),
        ]
    } else {
        vec![format!("flutter_rust_bridge:{}", env!("CARGO_PKG_VERSION"))]
    })?;

    // Temporarily avoid `^` before https://github.com/flutter/flutter/issues/84270 is fixed
    flutter_pub_add(&["ffigen:8.0.2".into(), "--dev".into()])?;
    if enable_integration_test {
        flutter_pub_add(&[
            "integration_test".into(),
            "--dev".into(),
            "--sdk=flutter".into(),
        ])?;
    }

    Ok(())
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
