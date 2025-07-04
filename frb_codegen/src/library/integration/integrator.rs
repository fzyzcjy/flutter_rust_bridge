use crate::integration::utils::{overlay_dir, replace_file_content};
use crate::library::commands::dart_fix::dart_fix;
use crate::library::commands::dart_format::dart_format;
use crate::library::commands::flutter::{flutter_pub_add, flutter_pub_get};
use crate::misc::Template;
use crate::utils::dart_repository::get_dart_package_name;
use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir};
use itertools::Itertools;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub struct IntegrateConfig {
    pub enable_overlay_lib: bool,
    pub enable_integration_test: bool,
    pub enable_dart_fix: bool,
    pub enable_dart_format: bool,
    pub enable_local_dependency: bool,
    pub rust_crate_name: Option<String>,
    pub rust_crate_dir: String,
    pub template: Template,
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

    info!("Overlay template onto project");
    let replacements = compute_replacements(&config, &dart_package_name, &rust_crate_name);
    match config.template {
        Template::App => {
            execute_overlay_dir(&TemplateDirs::APP, &replacements, &dart_root, &config, None)?
        }
        Template::Plugin => execute_overlay_dir(
            &TemplateDirs::PLUGIN,
            &replacements,
            &dart_root,
            &config,
            None,
        )?,
    }
    if config.enable_overlay_lib {
        execute_overlay_dir(
            &TemplateDirs::SHARED_LIB,
            &replacements,
            &dart_root,
            &config,
            None,
        )?;
        match config.template {
            Template::App => execute_overlay_dir(
                &TemplateDirs::APP_LIB,
                &replacements,
                &dart_root,
                &config,
                Some(&["main.dart".to_string()]),
            )?,
            Template::Plugin => execute_overlay_dir(
                &TemplateDirs::PLUGIN_LIB,
                &replacements,
                &dart_root,
                &config,
                Some(&[format!("{dart_package_name}.dart")]),
            )?,
        }
    }
    if config.enable_integration_test && config.template == Template::App {
        execute_overlay_dir(
            &TemplateDirs::APP_TEST,
            &replacements,
            &dart_root,
            &config,
            None,
        )?;
    }

    if config.enable_local_dependency && config.template == Template::Plugin {
        add_publish_to_none(&dart_root)?;
    }

    info!("Modify file permissions");
    modify_permissions(&dart_root, &config.template)?;

    info!("Add pub dependencies");
    pub_add_dependencies(
        config.enable_integration_test,
        config.enable_local_dependency,
        &rust_crate_name,
        &config.template,
    )?;

    info!("Setup cargokit dependencies");
    setup_cargokit_dependencies(&dart_root, &config.template)?;

    if config.enable_dart_fix {
        info!("Apply Dart fixes");
        dart_fix(&dart_root)?;
    } else {
        info!("Dart fixes is disabled.")
    }

    if config.enable_dart_format {
        info!("Format Dart code");
        dart_format(&dart_root, 80)?;
    } else {
        info!("Dart formats is disabled.");
    }

    Ok(())
}

fn execute_overlay_dir(
    current_reference_dir: &Dir,
    replacements: &HashMap<&'static str, &str>,
    dart_root: &Path,
    config: &IntegrateConfig,
    comment_out_files: Option<&[String]>,
) -> Result<()> {
    overlay_dir(
        current_reference_dir,
        replacements,
        dart_root,
        &|target_path, reference_content, existing_content| {
            modify_file(
                target_path.into(),
                reference_content,
                existing_content,
                replacements,
                config.enable_local_dependency,
                comment_out_files,
            )
        },
        &|path| filter_file(path, config.enable_integration_test),
    )
}

fn compute_replacements<'a>(
    config: &'a IntegrateConfig,
    dart_package_name: &'a str,
    rust_crate_name: &'a str,
) -> HashMap<&'static str, &'a str> {
    let mut replacements = HashMap::new();
    replacements.insert("REPLACE_ME_DART_PACKAGE_NAME", dart_package_name);
    match &config.template {
        Template::App => {
            replacements.insert("REPLACE_ME_RUST_CRATE_NAME", rust_crate_name);
        }
        Template::Plugin => {
            replacements.insert("REPLACE_ME_RUST_CRATE_NAME", dart_package_name);
        }
    }
    replacements.insert("REPLACE_ME_RUST_CRATE_DIR", config.rust_crate_dir.as_str());
    replacements.insert("REPLACE_ME_FRB_VERSION", env!("CARGO_PKG_VERSION"));

    let rust_frb_dependency = if config.enable_local_dependency {
        r#"{ path = "../../../frb_rust" }"#
    } else {
        concat!(r#""="#, env!("CARGO_PKG_VERSION"), r#"""#)
    };
    replacements.insert("REPLACE_ME_RUST_FRB_DEPENDENCY", rust_frb_dependency);

    replacements.insert("Cargo.toml.template", "Cargo.toml");
    replacements.insert("Cargo.lock.template", "Cargo.lock");

    replacements
}

fn modify_permissions(dart_root: &Path, template: &Template) -> Result<()> {
    #[allow(unused_variables)] // unused when in windows
    let dir_cargokit = match template {
        Template::App => dart_root.join("rust_builder").join("cargokit"),
        Template::Plugin => dart_root.join("cargokit"),
    };

    #[cfg(unix)]
    {
        set_permission_executable(&dir_cargokit.join("build_pod.sh"))?;
        set_permission_executable(&dir_cargokit.join("run_build_tool.sh"))?;
        set_permission_executable(&dir_cargokit.join("run_build_tool.cmd"))?;
    }

    Ok(())
}

fn setup_cargokit_dependencies(dart_root: &Path, template: &Template) -> Result<()> {
    let build_tool_dir = match template {
        Template::App => dart_root
            .join("rust_builder")
            .join("cargokit")
            .join("build_tool"),
        Template::Plugin => dart_root.join("cargokit").join("build_tool"),
    };

    flutter_pub_get(&build_tool_dir)
}

#[cfg(unix)]
fn set_permission_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    debug!("Change \"{}\" to executable", path.display());
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}

fn modify_file(
    target_path: PathBuf,
    reference_content: &[u8],
    existing_content: Option<Vec<u8>>,
    replacements: &HashMap<&str, &str>,
    enable_local_dependency: bool,
    comment_out_files: Option<&[String]>,
) -> Option<(PathBuf, Vec<u8>)> {
    let src = replace_file_content(reference_content, replacements);

    if let Some(existing_content) = existing_content {
        if let (Some(file_name), Some(files)) = (
            target_path.file_name().and_then(|e| e.to_str()),
            comment_out_files,
        ) {
            if files.contains(&file_name.to_owned()) {
                return comment_out_existing_file_and_write_template(
                    existing_content,
                    target_path,
                    &src,
                );
            }
        }
        // We do not care about this warning
        // frb-coverage:ignore-start
        warn!(
            "Skip writing to {target_path:?} because file already exists. \
            It is suggested to remove that file before running this command to apply the full template."
        );
        return None;
        // frb-coverage:ignore-end
    }

    if target_path.iter().contains(&OsStr::new("cargokit")) {
        if let Some(comments) = compute_cargokit_comments(&target_path) {
            return Some((target_path, [comments.as_bytes(), &src].concat()));
        }
    }

    if target_path
        .iter()
        .contains(&OsStr::new("flutter_rust_bridge.yaml"))
    {
        let mut ans = String::from_utf8(src).unwrap();
        if enable_local_dependency {
            ans += "\nlocal: true\n";
        }
        return Some((target_path, ans.as_bytes().to_owned()));
    }

    Some((target_path, src))
}

fn comment_out_existing_file_and_write_template(
    existing_content: Vec<u8>,
    path: PathBuf,
    src: &[u8],
) -> Option<(PathBuf, Vec<u8>)> {
    let existing_content = String::from_utf8(existing_content);
    let commented_existing_content = existing_content
        .map(|x| {
            format!(
                "// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.\n\n{}\n\n",
                x.split('\n').map(|line| format!("// {line}")).join("\n")
            )
        })
        .unwrap_or_default();
    Some((path, [commented_existing_content.as_bytes(), src].concat()))
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
    template: &Template,
) -> Result<()> {
    // frb-coverage:ignore-end
    match template {
        Template::App => flutter_pub_add(&[rust_crate_name, "--path=rust_builder"], None)?,
        Template::Plugin => flutter_pub_add(
            &["integration_test", "--dev", "--sdk=flutter"],
            Some(Path::new("example")),
        )?,
    }

    pub_add_dependency_frb(enable_local_dependency, None)?;

    // // Temporarily avoid `^` before https://github.com/flutter/flutter/issues/84270 is fixed
    // flutter_pub_add(&["ffigen:8.0.2", "--dev"])?;

    if enable_integration_test {
        flutter_pub_add(&["integration_test", "--dev", "--sdk=flutter"], None)?;
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
    if enable_local_dependency {
        flutter_pub_add(&["flutter_rust_bridge", "--path=../../frb_dart"], pwd)?;
    } else {
        flutter_pub_add(
            &[concat!("flutter_rust_bridge:", env!("CARGO_PKG_VERSION"))],
            pwd,
        )?;
    };

    Ok(())
}

fn add_publish_to_none(dart_root: &Path) -> Result<()> {
    let path = dart_root.join("pubspec.yaml");
    let text_raw = std::fs::read_to_string(&path)?;
    let text_output = format!("publish_to: none\n{text_raw}");
    std::fs::write(&path, text_output)?;
    Ok(())
}

struct TemplateDirs;

impl TemplateDirs {
    const SHARED_LIB: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/shared_lib");
    const APP: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/app");
    const APP_LIB: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/app_lib");
    const APP_TEST: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/app_test");
    const PLUGIN: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/plugin");
    const PLUGIN_LIB: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/plugin_lib");
}
