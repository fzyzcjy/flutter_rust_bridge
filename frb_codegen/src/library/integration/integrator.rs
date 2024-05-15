use crate::integration::utils::extract_dir_and_modify;
use crate::library::commands::flutter::{flutter_pub_add, flutter_pub_get};
use crate::library::commands::format_dart::format_dart;
use crate::utils::dart_repository::get_dart_package_name;
use crate::utils::path_utils::{find_dart_package_dir, path_to_string};
use anyhow::Result;
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use log::{debug, warn};
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

static INTEGRATION_TEMPLATE_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template");

pub struct IntegrateConfig {
    pub enable_integration_test: bool,
    pub enable_local_dependency: bool,
    pub rust_crate_name: Option<String>,
    pub rust_crate_dir: String,
}

/// Integrate Rust into existing Flutter project.
// ref: https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/
pub fn integrate(config: IntegrateConfig) -> Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");

    let dart_package_name = get_dart_package_name(&dart_root)?;
    let rust_crate_name = config
        .rust_crate_name
        .clone()
        .unwrap_or(format!("rust_lib_{}", dart_package_name));

    extract_dir_and_modify(
        &INTEGRATION_TEMPLATE_DIR,
        &dart_root,
        &|path, src_raw, existing_content| {
            modify_file(
                path,
                src_raw,
                existing_content,
                &dart_package_name,
                &rust_crate_name,
                &config.rust_crate_dir,
                config.enable_local_dependency,
            )
        },
        &|path| filter_file(path, config.enable_integration_test),
    )?;

    modify_permissions(&dart_root)?;

    pub_add_dependencies(
        config.enable_integration_test,
        config.enable_local_dependency,
        &rust_crate_name,
    )?;

    setup_cargokit_dependencies(&dart_root)?;

    format_dart(&[dart_root.clone()], &dart_root, 80, &[])?;

    Ok(())
}

fn modify_permissions(dart_root: &Path) -> Result<()> {
    #[allow(unused_variables)] // unused when in windows
    let dir_cargokit = dart_root.join("rust_builder").join("cargokit");

    #[cfg(unix)]
    {
        set_permission_executable(&dir_cargokit.join("build_pod.sh"))?;
        set_permission_executable(&dir_cargokit.join("run_build_tool.sh"))?;
        set_permission_executable(&dir_cargokit.join("run_build_tool.cmd"))?;
    }

    Ok(())
}

fn setup_cargokit_dependencies(dart_root: &Path) -> Result<()> {
    let build_tool_dir = dart_root
        .join("rust_builder")
        .join("cargokit")
        .join("build_tool");

    flutter_pub_get(&build_tool_dir)
}

#[cfg(unix)]
fn set_permission_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}

fn modify_file(
    path_raw: &Path,
    src_raw: &[u8],
    existing_content: Option<Vec<u8>>,
    dart_package_name: &str,
    rust_crate_name: &str,
    rust_crate_dir: &str,
    enable_local_dependency: bool,
) -> Option<(PathBuf, Vec<u8>)> {
    let replace_content_config = ReplaceContentConfig {
        dart_package_name,
        rust_crate_name,
        rust_crate_dir,
        enable_local_dependency,
    };
    let src = replace_file_content(src_raw, &replace_content_config);

    let path = compute_effective_path(path_raw, &replace_content_config);

    if let Some(existing_content) = existing_content {
        if path.file_name() == Some(OsStr::new("main.dart")) {
            let existing_content = String::from_utf8(existing_content);
            let commented_existing_content = existing_content
                .map(|x| {
                    format!(
                        "// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.\n\n{}\n\n",
                        x.split('\n').map(|line| format!("// {line}")).join("\n")
                    )
                })
                .unwrap_or_default();
            return Some((path, [commented_existing_content.as_bytes(), &src].concat()));
            // We do not care about this warning
            // frb-coverage:ignore-start
        }

        warn!(
            "Skip writing to {path:?} because file already exists. \
            It is suggested to remove that file before running this command to apply the full template."
        );
        return None;
        // frb-coverage:ignore-end
    }

    if path.iter().contains(&OsStr::new("cargokit")) {
        if let Some(comments) = compute_cargokit_comments(&path) {
            return Some((path, [comments.as_bytes(), &src].concat()));
        }
    }

    if path
        .iter()
        .contains(&OsStr::new("flutter_rust_bridge.yaml"))
    {
        let mut ans = String::from_utf8(src).unwrap();
        if enable_local_dependency {
            ans += "\nlocal: true\n";
        }
        return Some((path, ans.as_bytes().to_owned()));
    }

    Some((path, src))
}

fn compute_effective_path(path: &Path, config: &ReplaceContentConfig) -> PathBuf {
    let mut path = path.to_owned();
    if (path.extension().unwrap_or_default().to_str()).unwrap_or_default() == "template" {
        path = path.with_extension("")
    }
    path = replace_string_content(&path_to_string(&path).unwrap(), config).into();
    path
}

fn replace_file_content(raw: &[u8], config: &ReplaceContentConfig) -> Vec<u8> {
    match String::from_utf8(raw.to_owned()) {
        Ok(raw_str) => replace_string_content(&raw_str, config).into_bytes(),
        Err(e) => e.into_bytes(),
    }
}

struct ReplaceContentConfig<'a> {
    dart_package_name: &'a str,
    rust_crate_name: &'a str,
    rust_crate_dir: &'a str,
    enable_local_dependency: bool,
}

fn replace_string_content(raw: &str, config: &ReplaceContentConfig) -> String {
    raw.replace("REPLACE_ME_DART_PACKAGE_NAME", config.dart_package_name)
        .replace("REPLACE_ME_RUST_CRATE_NAME", config.rust_crate_name)
        .replace("REPLACE_ME_RUST_CRATE_DIR", config.rust_crate_dir)
        .replace("REPLACE_ME_FRB_VERSION", env!("CARGO_PKG_VERSION"))
        .replace(
            "REPLACE_ME_RUST_FRB_DEPENDENCY",
            &if config.enable_local_dependency {
                r#"{ path = "../../../frb_rust" }"#.to_owned()
            } else {
                format!(r#""={}""#, env!("CARGO_PKG_VERSION"))
            },
        )
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
        // frb-coverage:ignore-start
        ext => unreachable!("unexpected file extension for path={:?} ext={}", path, ext),
        // frb-coverage:ignore-end
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

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn pub_add_dependencies(
    enable_integration_test: bool,
    enable_local_dependency: bool,
    rust_crate_name: &str,
) -> Result<()> {
    // frb-coverage:ignore-end
    flutter_pub_add(
        &[rust_crate_name.into(), "--path=rust_builder".into()],
        None,
    )?;

    pub_add_dependency_frb(enable_local_dependency, None)?;

    // // Temporarily avoid `^` before https://github.com/flutter/flutter/issues/84270 is fixed
    // flutter_pub_add(&["ffigen:8.0.2".into(), "--dev".into()])?;

    if enable_integration_test {
        flutter_pub_add(
            &[
                "integration_test".into(),
                "--dev".into(),
                "--sdk=flutter".into(),
            ],
            None,
        )?;
        // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
        // frb-coverage:ignore-start
    }
    // frb-coverage:ignore-end

    Ok(())
}

pub(crate) fn pub_add_dependency_frb(
    enable_local_dependency: bool,
    pwd: Option<&Path>,
) -> Result<()> {
    flutter_pub_add(
        &if enable_local_dependency {
            vec![
                "flutter_rust_bridge".to_owned(),
                "--path=../../frb_dart".to_owned(),
            ]
        } else {
            vec![format!("flutter_rust_bridge:{}", env!("CARGO_PKG_VERSION"))]
        },
        pwd,
    )?;
    Ok(())
}
