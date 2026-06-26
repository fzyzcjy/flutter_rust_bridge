use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use anyhow::{bail, Context};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn clean(config: &PolisherInternalConfig) -> anyhow::Result<()> {
    if !config.pre_generation_cleanup {
        return Ok(());
    }

    clean_dart_output(
        &config.dart_output,
        &config.dart_root,
        &config.rust_crate_dir,
    )?;
    remove_file_if_exists(&config.rust_output_path)?;
    if let Some(c_output_path) = &config.c_output_path {
        remove_file_if_exists(c_output_path)?;
    }
    for duplicated_c_output_path in &config.duplicated_c_output_path {
        remove_file_if_exists(duplicated_c_output_path)?;
    }

    Ok(())
}

fn clean_dart_output(
    dart_output: &Path,
    dart_root: &Path,
    rust_crate_dir: &Path,
) -> anyhow::Result<()> {
    if !dart_output.exists() {
        return Ok(());
    }

    ensure_safe_dart_output(dart_output, dart_root, rust_crate_dir)?;
    let entries = fs::read_dir(dart_output)?.collect::<Result<Vec<_>, _>>()?;
    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() || (file_type.is_symlink() && path.is_dir()) {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

fn ensure_safe_dart_output(
    dart_output: &Path,
    dart_root: &Path,
    rust_crate_dir: &Path,
) -> anyhow::Result<()> {
    let dart_output = dart_output
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize dart_output path {dart_output:?}"))?;
    let dart_root = dart_root
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize dart_root path {dart_root:?}"))?;
    let rust_crate_dir = rust_crate_dir.canonicalize().with_context(|| {
        format!("Failed to canonicalize rust_crate_dir path {rust_crate_dir:?}")
    })?;
    let dart_lib = dart_root.join("lib");
    let dart_lib_src = dart_lib.join("src");
    let rust_src = canonicalize_if_exists(rust_crate_dir.join("src"));
    let protected_dart_dirs = [
        canonicalize_if_exists(dart_lib),
        canonicalize_if_exists(dart_lib_src),
    ];
    let protected_dart_dir_prefixes = [
        dart_root.join(".dart_tool"),
        dart_root.join(".git"),
        dart_root.join(".github"),
        dart_root.join("android"),
        dart_root.join("bin"),
        dart_root.join("build"),
        dart_root.join("example"),
        dart_root.join("integration_test"),
        dart_root.join("ios"),
        dart_root.join("linux"),
        dart_root.join("macos"),
        dart_root.join("test"),
        dart_root.join("tool"),
        dart_root.join("web"),
        dart_root.join("windows"),
    ]
    .map(canonicalize_if_exists);

    if dart_output.parent().is_none()
        || !dart_output.starts_with(&dart_root)
        || dart_output == dart_root
        || protected_dart_dirs.contains(&dart_output)
        || protected_dart_dir_prefixes
            .iter()
            .any(|protected_dir| dart_output.starts_with(protected_dir))
        || dart_output.starts_with(&rust_src)
        || (dart_output.starts_with(&rust_crate_dir) && !dart_root.starts_with(&rust_crate_dir))
    {
        bail!("pre_generation_cleanup refuses to clean unsafe dart_output path: {dart_output:?}");
    }

    Ok(())
}

fn canonicalize_if_exists(path: PathBuf) -> PathBuf {
    path.canonicalize().unwrap_or(path)
}

fn remove_file_if_exists(path: &Path) -> anyhow::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("Failed to remove {path:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::clean;
    use crate::codegen::polisher::internal_config::PolisherInternalConfig;
    use crate::misc::FvmInstallMode;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_clean_skips_when_disabled() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let config = create_config(temp_dir.path(), false)?;
        let stale_dart_file = config.dart_output.join("stale.dart");

        clean(&config)?;

        assert!(stale_dart_file.exists());
        assert!(config.rust_output_path.exists());
        assert!(config.c_output_path.as_ref().unwrap().exists());
        Ok(())
    }

    #[test]
    fn test_clean_removes_generated_outputs_when_enabled() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let config = create_config(temp_dir.path(), true)?;
        let stale_dart_file = config.dart_output.join("stale.dart");
        let stale_nested_file = config.dart_output.join("nested").join("stale.dart");
        let duplicated_c_output_path = config.duplicated_c_output_path[0].clone();

        clean(&config)?;

        assert!(!stale_dart_file.exists());
        assert!(!stale_nested_file.exists());
        assert!(!config.rust_output_path.exists());
        assert!(!config.c_output_path.as_ref().unwrap().exists());
        assert!(!duplicated_c_output_path.exists());
        assert!(config.dart_output.exists());
        Ok(())
    }

    #[test]
    fn test_clean_rejects_dart_root_output() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.dart_output = config.dart_root.clone();

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_rejects_dart_output_outside_dart_root() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.dart_output = temp_dir.path().join("external");
        fs::create_dir_all(&config.dart_output)?;

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_rejects_lib_output() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.dart_output = config.dart_root.join("lib");

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_rejects_lib_src_output() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.dart_output = config.dart_root.join("lib").join("src");

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_rejects_platform_subdirectory_output() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.dart_output = config.dart_root.join("ios").join("Runner");
        fs::create_dir_all(&config.dart_output)?;

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn test_clean_rejects_canonicalized_platform_symlink_output() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        let real_ios = config.dart_root.join("real_ios");
        fs::create_dir_all(real_ios.join("Runner"))?;
        std::os::unix::fs::symlink(&real_ios, config.dart_root.join("ios"))?;
        config.dart_output = config.dart_root.join("ios").join("Runner");

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_rejects_project_metadata_subdirectory_output() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.dart_output = config.dart_root.join(".dart_tool").join("flutter_build");
        fs::create_dir_all(&config.dart_output)?;

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_rejects_tool_subdirectory_output() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.dart_output = config.dart_root.join("tool").join("build_scripts");
        fs::create_dir_all(&config.dart_output)?;

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_rejects_dart_output_inside_rust_crate() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.rust_crate_dir = config.dart_root.join("rust");
        config.dart_output = config.rust_crate_dir.join("generated");
        fs::create_dir_all(&config.dart_output)?;

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_rejects_rust_src_output_when_dart_root_is_rust_crate() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.rust_crate_dir = temp_dir.path().join("app");
        config.dart_root = config.rust_crate_dir.clone();
        config.dart_output = config.rust_crate_dir.join("src").join("generated");
        fs::create_dir_all(&config.dart_output)?;

        let result = clean(&config);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unsafe dart_output path"));
        Ok(())
    }

    #[test]
    fn test_clean_allows_dart_root_inside_rust_crate() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let mut config = create_config(temp_dir.path(), true)?;
        config.rust_crate_dir = temp_dir.path().join("rust");
        config.dart_root = config.rust_crate_dir.join("dart");
        config.dart_output = config.dart_root.join("lib").join("src").join("rust");
        fs::create_dir_all(config.dart_output.join("nested"))?;
        fs::write(config.dart_output.join("stale.dart"), "stale")?;
        fs::write(
            config.dart_output.join("nested").join("stale.dart"),
            "stale",
        )?;

        clean(&config)?;

        assert!(!config.dart_output.join("stale.dart").exists());
        assert!(!config
            .dart_output
            .join("nested")
            .join("stale.dart")
            .exists());
        Ok(())
    }

    fn create_config(
        root: &Path,
        pre_generation_cleanup: bool,
    ) -> anyhow::Result<PolisherInternalConfig> {
        let dart_root = root.join("dart");
        let dart_output = dart_root.join("lib").join("src").join("rust");
        let rust_crate_dir = root.join("rust");
        let duplicated_c_output_path = dart_root.join("duplicate.h");
        fs::create_dir_all(dart_output.join("nested"))?;
        fs::create_dir_all(rust_crate_dir.join("src"))?;
        fs::write(dart_output.join("stale.dart"), "stale")?;
        fs::write(dart_output.join("nested").join("stale.dart"), "stale")?;
        fs::write(rust_crate_dir.join("src").join("frb_generated.rs"), "stale")?;
        fs::write(dart_root.join("bridge.h"), "stale")?;
        fs::write(&duplicated_c_output_path, "stale")?;

        Ok(PolisherInternalConfig {
            duplicated_c_output_path: vec![duplicated_c_output_path],
            dart_format_line_length: 80,
            dart_format: true,
            dart_fix: true,
            rust_format: true,
            add_mod_to_lib: true,
            build_runner: true,
            web_enabled: true,
            dart_output,
            dart_root,
            rust_crate_dir: rust_crate_dir.clone(),
            rust_output_path: rust_crate_dir.join("src").join("frb_generated.rs"),
            c_output_path: Some(root.join("dart").join("bridge.h")),
            enable_auto_upgrade: true,
            fvm_install_mode: FvmInstallMode::Normal,
            pre_generation_cleanup,
        })
    }
}
