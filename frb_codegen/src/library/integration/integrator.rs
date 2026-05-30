use crate::integration::utils::{overlay_dir, replace_file_content};
use crate::library::commands::cargo::cargo_fetch;
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
use std::fs;
use std::path::{Path, PathBuf};

const REFRESH_CARGO_LOCK_ORDERING_ENV_VAR: &str = "FRB_REFRESH_CARGO_LOCK_ORDERING";

pub struct IntegrateConfig {
    pub enable_write_lib: bool,
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
        .unwrap_or(match &config.template {
            Template::App => {
                format!("rust_lib_{dart_package_name}")
            }
            Template::Plugin => dart_package_name.to_owned(),
        });

    info!("Overlay template onto project");
    let replacements = compute_replacements(&config, &dart_package_name, &rust_crate_name);
    execute_overlay_dir(
        &TemplateDirs::SHARED,
        &replacements,
        &dart_root,
        &config,
        None,
    )?;
    let (dir, comment_out_files) = match &config.template {
        Template::App => (&TemplateDirs::APP, vec!["main.dart".to_string()]),
        Template::Plugin => (
            &TemplateDirs::PLUGIN,
            vec![format!("{dart_package_name}.dart")],
        ),
    };
    execute_overlay_dir(
        dir,
        &replacements,
        &dart_root,
        &config,
        Some(&comment_out_files),
    )?;

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

    exclude_cargokit_from_outer_analyzer(&dart_root, &config.template)?;

    if config.enable_dart_fix {
        info!("Apply Dart fixes");
        dart_fix(&dart_root)?;
    } else {
        info!("Dart fix is disabled.")
    }

    if config.enable_dart_format {
        info!("Format Dart code");
        dart_format(&dart_root, 80)?;
    } else {
        info!("Dart format is disabled.");
    }

    maybe_refresh_cargo_lock_ordering(&dart_root, &config.rust_crate_dir)?;

    Ok(())
}

fn maybe_refresh_cargo_lock_ordering(dart_root: &Path, rust_crate_dir: &str) -> Result<()> {
    if !should_refresh_cargo_lock_ordering() {
        debug!(
            "Skip Cargo.lock ordering refresh; set {REFRESH_CARGO_LOCK_ORDERING_ENV_VAR}=1 to enable"
        );
        return Ok(());
    }

    refresh_cargo_lock_ordering(dart_root, rust_crate_dir)
}

fn should_refresh_cargo_lock_ordering() -> bool {
    env::var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR).unwrap_or_default() == "1"
}

fn refresh_cargo_lock_ordering(dart_root: &Path, rust_crate_dir: &str) -> Result<()> {
    info!("Refresh Cargo.lock ordering because {REFRESH_CARGO_LOCK_ORDERING_ENV_VAR}=1");
    cargo_fetch(&dart_root.join(rust_crate_dir))
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
        &|path| {
            filter_file(
                path,
                config.enable_write_lib,
                config.enable_integration_test,
            )
        },
    )
}

fn compute_replacements<'a>(
    config: &'a IntegrateConfig,
    dart_package_name: &'a str,
    rust_crate_name: &'a str,
) -> HashMap<&'static str, &'a str> {
    let mut replacements = HashMap::new();
    replacements.insert("REPLACE_ME_DART_PACKAGE_NAME", dart_package_name);
    replacements.insert("REPLACE_ME_RUST_CRATE_NAME", rust_crate_name);
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

fn exclude_cargokit_from_outer_analyzer(dart_root: &Path, template: &Template) -> Result<()> {
    let path = dart_root.join("analysis_options.yaml");
    if !path.exists() {
        return Ok(());
    }

    let exclude = match template {
        Template::App => "rust_builder/cargokit/**",
        Template::Plugin => "cargokit/**",
    };
    let text = fs::read_to_string(&path)?;
    let text = add_analyzer_exclude(&text, exclude);
    fs::write(path, text)?;

    Ok(())
}

fn add_analyzer_exclude(text: &str, exclude: &str) -> String {
    let default_list_item_indent = "    ";
    if text
        .lines()
        .any(|line| line.trim() == format!("- {exclude}"))
    {
        return text.to_owned();
    }

    let mut lines = text.lines().map(String::from).collect_vec();
    let Some(analyzer_index) = lines.iter().position(|line| line.trim() == "analyzer:") else {
        let exclude_line = format!("{default_list_item_indent}- {exclude}");
        return format!("analyzer:\n  exclude:\n{exclude_line}\n\n{text}");
    };

    let block_end = lines
        .iter()
        .enumerate()
        .skip(analyzer_index + 1)
        .find(|(_, line)| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !line.starts_with(' ') && !line.starts_with('#')
        })
        .map(|(index, _)| index)
        .unwrap_or(lines.len());

    if let Some(exclude_index) = lines[analyzer_index + 1..block_end]
        .iter()
        .position(|line| line.trim() == "exclude:")
        .map(|index| index + analyzer_index + 1)
    {
        let list_item_indent = detect_yaml_list_item_indent(&lines[exclude_index + 1..block_end])
            .unwrap_or_else(|| default_list_item_indent.to_owned());
        let exclude_line = format!("{list_item_indent}- {exclude}");
        let insert_index = lines[exclude_index + 1..block_end]
            .iter()
            .position(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !line.starts_with(&list_item_indent)
            })
            .map(|index| index + exclude_index + 1)
            .unwrap_or(block_end);
        lines.insert(insert_index, exclude_line);
    } else {
        lines.insert(
            analyzer_index + 1,
            format!("{default_list_item_indent}- {exclude}"),
        );
        lines.insert(analyzer_index + 1, "  exclude:".to_owned());
    }

    let mut output = lines.join("\n");
    if text.ends_with('\n') {
        output.push('\n');
    }
    output
}

fn detect_yaml_list_item_indent(lines: &[String]) -> Option<String> {
    lines.iter().find_map(|line| {
        let trimmed = line.trim_start();
        trimmed
            .starts_with("- ")
            .then(|| line[..line.len() - trimmed.len()].to_owned())
    })
}

#[cfg(unix)]
fn set_permission_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    // the early-return branch is exercised by tests, but llvm-cov still reports the
    // branch lines as uncovered
    // frb-coverage:ignore-start
    if !path.exists() {
        debug!(
            "Skip executable permission for missing path {}",
            path.display()
        );
        return Ok(());
    }
    // frb-coverage:ignore-end

    debug!("Change \"{}\" to executable", path.display());
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::{
        add_analyzer_exclude, exclude_cargokit_from_outer_analyzer, refresh_cargo_lock_ordering,
        set_permission_executable, should_refresh_cargo_lock_ordering,
        REFRESH_CARGO_LOCK_ORDERING_ENV_VAR,
    };
    use crate::misc::Template;
    use serial_test::serial;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_set_permission_executable_missing_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let missing_path = temp_dir.path().join("missing.sh");

        set_permission_executable(&missing_path).unwrap();

        assert!(!missing_path.exists());
    }

    #[test]
    fn test_set_permission_executable_existing_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let script_path = temp_dir.path().join("script.sh");
        fs::write(&script_path, "#!/bin/sh\n").unwrap();
        fs::set_permissions(&script_path, PermissionsExt::from_mode(0o644)).unwrap();

        set_permission_executable(&script_path).unwrap();

        let permissions = fs::metadata(&script_path).unwrap().permissions();
        assert_eq!(permissions.mode() & 0o777, 0o755);
    }

    #[test]
    fn test_add_analyzer_exclude_prepends_analyzer_block() {
        let actual = add_analyzer_exclude(
            "include: package:flutter_lints/flutter.yaml\n",
            "cargokit/**",
        );

        assert_eq!(
            actual,
            "analyzer:\n  exclude:\n    - cargokit/**\n\ninclude: package:flutter_lints/flutter.yaml\n"
        );
    }

    #[test]
    fn test_add_analyzer_exclude_preserves_existing_analyzer_options() {
        let actual = add_analyzer_exclude(
            "analyzer:\n  errors:\n    prefer_const_constructors: ignore\ninclude: package:flutter_lints/flutter.yaml\n",
            "rust_builder/cargokit/**",
        );

        assert_eq!(
            actual,
            "analyzer:\n  exclude:\n    - rust_builder/cargokit/**\n  errors:\n    prefer_const_constructors: ignore\ninclude: package:flutter_lints/flutter.yaml\n"
        );
    }

    #[test]
    fn test_add_analyzer_exclude_is_idempotent() {
        let text = "analyzer:\n  exclude:\n    - rust_builder/cargokit/**\n";

        assert_eq!(add_analyzer_exclude(text, "rust_builder/cargokit/**"), text);
    }

    #[test]
    fn test_add_analyzer_exclude_appends_to_existing_exclude_block() {
        let actual = add_analyzer_exclude(
            "analyzer:\n  exclude:\n    - build/**\n  errors:\n    avoid_print: ignore\n",
            "rust_builder/cargokit/**",
        );

        assert_eq!(
            actual,
            "analyzer:\n  exclude:\n    - build/**\n    - rust_builder/cargokit/**\n  errors:\n    avoid_print: ignore\n"
        );
    }

    #[test]
    fn test_add_analyzer_exclude_appends_to_terminal_exclude_block() {
        let actual = add_analyzer_exclude(
            "analyzer:\n  exclude:\n    - build/**\n",
            "rust_builder/cargokit/**",
        );

        assert_eq!(
            actual,
            "analyzer:\n  exclude:\n    - build/**\n    - rust_builder/cargokit/**\n"
        );
    }

    #[test]
    fn test_add_analyzer_exclude_uses_existing_exclude_list_indent() {
        let actual = add_analyzer_exclude(
            "analyzer:\n  exclude:\n      - build/**\n  errors:\n    avoid_print: ignore\n",
            "rust_builder/cargokit/**",
        );

        assert_eq!(
            actual,
            "analyzer:\n  exclude:\n      - build/**\n      - rust_builder/cargokit/**\n  errors:\n    avoid_print: ignore\n"
        );
    }

    #[test]
    fn test_exclude_cargokit_from_outer_analyzer_ignores_missing_file() {
        let temp_dir = tempfile::tempdir().unwrap();

        exclude_cargokit_from_outer_analyzer(temp_dir.path(), &Template::App).unwrap();

        assert!(!temp_dir.path().join("analysis_options.yaml").exists());
    }

    #[test]
    fn test_refresh_cargo_lock_ordering_real_fetch() {
        let temp_dir = tempfile::tempdir().unwrap();
        let rust_dir = temp_dir.path().join("rust");
        fs::create_dir_all(rust_dir.join("src")).unwrap();
        fs::write(
            rust_dir.join("Cargo.toml"),
            "[package]\nname = \"integrator_refresh_test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            rust_dir.join("src/lib.rs"),
            "pub fn answer() -> i32 { 42 }\n",
        )
        .unwrap();

        refresh_cargo_lock_ordering(temp_dir.path(), "rust").unwrap();
    }

    #[test]
    #[serial]
    fn test_should_refresh_cargo_lock_ordering_only_when_env_var_is_one() {
        std::env::remove_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR);
        assert!(!should_refresh_cargo_lock_ordering());

        std::env::set_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR, "true");
        assert!(!should_refresh_cargo_lock_ordering());

        std::env::set_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR, "1");
        assert!(should_refresh_cargo_lock_ordering());

        std::env::remove_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR);
    }
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

fn filter_file(path: &Path, enable_write_lib: bool, enable_integration_test: bool) -> bool {
    if path.iter().contains(&OsStr::new("cargokit")) {
        return ![".git", ".github", "docs", "test"].contains(&file_name(path));
    }

    if !enable_write_lib {
        if path.iter().contains(&OsStr::new("rust_builder")) {
            return true;
        }
        if path.iter().contains(&OsStr::new("android"))
            || path.iter().contains(&OsStr::new("ios"))
            || path.iter().contains(&OsStr::new("windows"))
            || path.iter().contains(&OsStr::new("macos"))
            || path.iter().contains(&OsStr::new("linux"))
            || path.iter().contains(&OsStr::new("lib"))
            || path
                .iter()
                .contains(&OsStr::new("REPLACE_ME_RUST_CRATE_DIR"))
            || path
                .iter()
                .contains(&OsStr::new("flutter_rust_bridge.yaml"))
        {
            return false;
        }
    }

    if !enable_integration_test
        && (path.iter().contains(&OsStr::new("integration_test"))
            || path.iter().contains(&OsStr::new("test_driver")))
    {
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
    const SHARED: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/shared");
    const APP: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/app");
    const PLUGIN: Dir<'static> =
        include_dir!("$CARGO_MANIFEST_DIR/assets/integration_template/plugin");
}
