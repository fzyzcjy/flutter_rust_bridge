use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use anyhow::{bail, Context};
use std::fs;
use std::path::Path;

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
    for entry in fs::read_dir(dart_output)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
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
    let dart_output = dart_output.canonicalize()?;
    let dart_root = dart_root.canonicalize()?;
    let rust_crate_dir = rust_crate_dir.canonicalize()?;
    let dart_lib = dart_root.join("lib");
    let dart_lib_src = dart_lib.join("src");

    if dart_output.parent().is_none()
        || !dart_output.starts_with(&dart_root)
        || dart_output == dart_root
        || dart_output == dart_lib
        || dart_output == dart_lib_src
        || dart_output == rust_crate_dir
    {
        bail!("pre_generation_cleanup refuses to clean unsafe dart_output path: {dart_output:?}");
    }

    Ok(())
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
