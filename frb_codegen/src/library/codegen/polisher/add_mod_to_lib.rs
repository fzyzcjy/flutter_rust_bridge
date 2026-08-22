use anyhow::*;
use log::{info, warn};
use pathdiff::diff_paths;
use std::fs;
use std::path::{Component, Path, PathBuf};

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
pub(super) fn try_add_mod_to_lib(rust_crate_dir: &Path, rust_output_path: &Path) {
    // frb-coverage:ignore-end
    if let Err(e) = auto_add_mod_to_lib_core(rust_crate_dir, rust_output_path) {
        // We do not care about the warning
        // frb-coverage:ignore-start
        warn!(
            "add_mod_to_lib fail, the generated code may or may not have problems. \
            Please ensure you have add code like `mod the_generated_bridge_code;` to your `lib.rs`. \
            Details: {}",
            e
        );
        // frb-coverage:ignore-end
    }
}

fn auto_add_mod_to_lib_core(rust_crate_dir: &Path, rust_output_path: &Path) -> Result<()> {
    let path_src_folder = rust_crate_dir.join("src");
    let rust_output_path_relative_to_src_folder_raw =
        diff_paths(rust_output_path, path_src_folder.clone()).with_context(|| {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            format!(
                "rust_output_path={rust_output_path:?} is unrelated to path_src_folder={path_src_folder:?}",
            )
            // frb-coverage:ignore-end
        })?;
    let rust_output_path_relative_to_src_folder =
        normalize_descendant_path(&rust_output_path_relative_to_src_folder_raw)?;

    let mod_name = rust_output_path_relative_to_src_folder
        .file_stem()
        .context("No file_stem")?
        .to_str()
        .context("Not a UTF-8 path")?
        .to_string()
        .replace('/', "::");
    let expect_code = format!("mod {mod_name};");

    let path_lib_rs = path_src_folder.join("lib.rs");

    let raw_content_lib_rs = fs::read_to_string(path_lib_rs.clone())?;
    if !raw_content_lib_rs.contains(&expect_code) {
        info!("Inject `{}` into {:?}", &expect_code, &path_lib_rs);

        let comments = " /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */";
        let modified_content_lib_rs = format!("{expect_code}{comments}\n{raw_content_lib_rs}");

        fs::write(&path_lib_rs, modified_content_lib_rs).unwrap();
    }

    Ok(())
}

fn normalize_descendant_path(path: &Path) -> Result<PathBuf> {
    let mut output = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => output.push(value),
            Component::CurDir => {}
            Component::ParentDir => ensure!(output.pop(), "path escapes its base directory"),
            Component::RootDir | Component::Prefix(_) => bail!("path is not relative"),
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::auto_add_mod_to_lib_core;
    use anyhow::Result;
    use std::fs;
    use tempfile::tempdir;

    /// Inserts the generated module declaration only once.
    #[test]
    fn test_add_mod_to_lib_inserts_declaration_once() -> Result<()> {
        let temp_dir = tempdir()?;
        let crate_dir = temp_dir.path().join("crate");
        let src_dir = crate_dir.join("src");
        fs::create_dir_all(&src_dir)?;
        fs::write(src_dir.join("lib.rs"), "pub fn existing() {}\n")?;
        let output = src_dir.join("bridge.rs");

        auto_add_mod_to_lib_core(&crate_dir, &output)?;
        auto_add_mod_to_lib_core(&crate_dir, &output)?;

        let content = fs::read_to_string(src_dir.join("lib.rs"))?;
        assert_eq!(content.matches("mod bridge;").count(), 1);
        Ok(())
    }

    /// Uses a nested generated filename as the module declaration name.
    #[test]
    fn test_add_mod_to_lib_handles_nested_output() -> Result<()> {
        let temp_dir = tempdir()?;
        let crate_dir = temp_dir.path().join("crate");
        let src_dir = crate_dir.join("src");
        fs::create_dir_all(src_dir.join("nested"))?;
        fs::write(src_dir.join("lib.rs"), "")?;

        auto_add_mod_to_lib_core(&crate_dir, &src_dir.join("nested/bridge.rs"))?;

        assert!(fs::read_to_string(src_dir.join("lib.rs"))?.starts_with("mod bridge;"));
        Ok(())
    }

    /// Accepts parent components that normalize within the source directory.
    #[test]
    fn test_add_mod_to_lib_normalizes_output_inside_src() -> Result<()> {
        let temp_dir = tempdir()?;
        let crate_dir = temp_dir.path().join("crate");
        let src_dir = crate_dir.join("src");
        fs::create_dir_all(src_dir.join("nested"))?;
        fs::write(src_dir.join("lib.rs"), "")?;

        auto_add_mod_to_lib_core(&crate_dir, &src_dir.join("nested/../bridge.rs"))?;

        assert!(fs::read_to_string(src_dir.join("lib.rs"))?.starts_with("mod bridge;"));
        Ok(())
    }

    /// Rejects an output path outside the crate source directory.
    #[test]
    fn test_add_mod_to_lib_rejects_output_outside_src() -> Result<()> {
        let temp_dir = tempdir()?;
        let crate_dir = temp_dir.path().join("crate");
        let src_dir = crate_dir.join("src");
        fs::create_dir_all(&src_dir)?;
        fs::write(src_dir.join("lib.rs"), "")?;

        assert!(auto_add_mod_to_lib_core(&crate_dir, &temp_dir.path().join("bridge.rs")).is_err());
        Ok(())
    }

    /// Propagates file-system errors when the crate has no lib.rs.
    #[test]
    fn test_add_mod_to_lib_propagates_missing_lib_error() -> Result<()> {
        let temp_dir = tempdir()?;
        let crate_dir = temp_dir.path().join("crate");
        fs::create_dir_all(crate_dir.join("src"))?;

        assert!(auto_add_mod_to_lib_core(&crate_dir, &crate_dir.join("src/bridge.rs")).is_err());
        Ok(())
    }
}
