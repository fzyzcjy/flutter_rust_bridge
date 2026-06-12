use crate::library::commands::flutter::flutter_pub_get;
use crate::misc::{FvmInstallMode, Template};
use anyhow::Result;
use itertools::Itertools;
use std::fs;
use std::path::Path;

pub(super) fn modify_permissions(dart_root: &Path, template: &Template) -> Result<()> {
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

pub(super) fn setup_cargokit_dependencies(
    dart_root: &Path,
    template: &Template,
    fvm_install_mode: FvmInstallMode,
) -> Result<()> {
    let build_tool_dir = match template {
        Template::App => dart_root
            .join("rust_builder")
            .join("cargokit")
            .join("build_tool"),
        Template::Plugin => dart_root.join("cargokit").join("build_tool"),
    };

    flutter_pub_get(&build_tool_dir, fvm_install_mode)
}

pub(super) fn exclude_cargokit_from_outer_analyzer(
    dart_root: &Path,
    template: &Template,
) -> Result<()> {
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
    // Use a targeted text edit so YAML comments, blank lines, and formatting stay unchanged.
    let exclude_line = format!("    - {exclude}");
    if text
        .lines()
        .any(|line| line.trim() == format!("- {exclude}"))
    {
        return text.to_owned();
    }

    let mut lines = text.lines().map(String::from).collect_vec();
    let Some(analyzer_index) = lines.iter().position(|line| line.trim() == "analyzer:") else {
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
        let insert_index = lines[exclude_index + 1..block_end]
            .iter()
            .position(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with("- ")
            })
            .map(|index| index + exclude_index + 1)
            .unwrap_or(block_end);
        lines.insert(insert_index, exclude_line);
    } else {
        lines.insert(analyzer_index + 1, exclude_line);
        lines.insert(analyzer_index + 1, "  exclude:".to_owned());
    }

    let mut output = lines.join("\n");
    if text.ends_with('\n') {
        output.push('\n');
    }
    output
}

#[cfg(unix)]
fn set_permission_executable(path: &Path) -> Result<()> {
    use log::debug;
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
        add_analyzer_exclude, exclude_cargokit_from_outer_analyzer, set_permission_executable,
    };
    use crate::misc::Template;
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
            r#"include: package:flutter_lints/flutter.yaml
"#,
            "cargokit/**",
        );

        assert_eq!(
            actual,
            r#"analyzer:
  exclude:
    - cargokit/**

include: package:flutter_lints/flutter.yaml
"#
        );
    }

    #[test]
    fn test_add_analyzer_exclude_preserves_existing_analyzer_options() {
        let actual = add_analyzer_exclude(
            r#"analyzer:
  errors:
    prefer_const_constructors: ignore
include: package:flutter_lints/flutter.yaml
"#,
            "rust_builder/cargokit/**",
        );

        assert_eq!(
            actual,
            r#"analyzer:
  exclude:
    - rust_builder/cargokit/**
  errors:
    prefer_const_constructors: ignore
include: package:flutter_lints/flutter.yaml
"#
        );
    }

    #[test]
    fn test_add_analyzer_exclude_is_idempotent() {
        let text = r#"analyzer:
  exclude:
    - rust_builder/cargokit/**
"#;

        assert_eq!(add_analyzer_exclude(text, "rust_builder/cargokit/**"), text);
    }

    #[test]
    fn test_add_analyzer_exclude_appends_to_existing_exclude_block() {
        let actual = add_analyzer_exclude(
            r#"analyzer:
  exclude:
    - build/**
  errors:
    avoid_print: ignore
"#,
            "rust_builder/cargokit/**",
        );

        assert_eq!(
            actual,
            r#"analyzer:
  exclude:
    - build/**
    - rust_builder/cargokit/**
  errors:
    avoid_print: ignore
"#
        );
    }

    #[test]
    fn test_add_analyzer_exclude_appends_to_terminal_exclude_block() {
        let actual = add_analyzer_exclude(
            r#"analyzer:
  exclude:
    - build/**
"#,
            "rust_builder/cargokit/**",
        );

        assert_eq!(
            actual,
            r#"analyzer:
  exclude:
    - build/**
    - rust_builder/cargokit/**
"#
        );
    }

    #[test]
    fn test_add_analyzer_exclude_preserves_missing_trailing_newline() {
        let actual = add_analyzer_exclude(
            r#"analyzer:
  exclude:
    - build/**"#,
            "rust_builder/cargokit/**",
        );

        assert_eq!(
            actual,
            r#"analyzer:
  exclude:
    - build/**
    - rust_builder/cargokit/**"#
        );
    }

    #[test]
    fn test_exclude_cargokit_from_outer_analyzer_ignores_missing_file() {
        let temp_dir = tempfile::tempdir().unwrap();

        exclude_cargokit_from_outer_analyzer(temp_dir.path(), &Template::App).unwrap();

        assert!(!temp_dir.path().join("analysis_options.yaml").exists());
    }

    #[test]
    fn test_exclude_cargokit_from_outer_analyzer_writes_app_exclude() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("analysis_options.yaml");
        fs::write(
            &path,
            r#"include: package:flutter_lints/flutter.yaml
"#,
        )
        .unwrap();

        exclude_cargokit_from_outer_analyzer(temp_dir.path(), &Template::App).unwrap();

        assert_eq!(
            fs::read_to_string(path).unwrap(),
            r#"analyzer:
  exclude:
    - rust_builder/cargokit/**

include: package:flutter_lints/flutter.yaml
"#
        );
    }

    #[test]
    fn test_exclude_cargokit_from_outer_analyzer_writes_plugin_exclude() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("analysis_options.yaml");
        fs::write(
            &path,
            r#"include: package:flutter_lints/flutter.yaml
"#,
        )
        .unwrap();

        exclude_cargokit_from_outer_analyzer(temp_dir.path(), &Template::Plugin).unwrap();

        assert_eq!(
            fs::read_to_string(path).unwrap(),
            r#"analyzer:
  exclude:
    - cargokit/**

include: package:flutter_lints/flutter.yaml
"#
        );
    }
}
