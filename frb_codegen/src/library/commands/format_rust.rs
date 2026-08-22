use crate::command_run;
use crate::library::commands::command_runner::{call_shell, check_exit_code};
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::Context;
use itertools::Itertools;
use log::debug;
use pathdiff::diff_paths;
use std::path::{Path, PathBuf};

pub fn format_rust(paths: &[PathBuf], base_path: &Path) -> anyhow::Result<()> {
    let paths = prepare_paths(paths, base_path, &[])?;
    debug!("execute format_rust paths={paths:?}");

    check_exit_code(&command_run!(
        call_shell[Some(base_path), None],
        "rustfmt",
        // otherwise cannot understand `async move`
        "--edition",
        "2018",
        *paths
    )?)
}

pub(super) fn prepare_paths(
    paths: &[PathBuf],
    base_path: &Path,
    extra_extensions: &[&str],
) -> anyhow::Result<Vec<PathBuf>> {
    let base_path_str = path_to_string(base_path)?;
    let normalized_base_path = normalize_windows_unc_path(&base_path_str);

    Ok(paths
        .iter()
        .map(|path| {
            let mut path: PathBuf = normalize_windows_unc_path(&path_to_string(path)?)
                .to_owned()
                .into();
            path = diff_paths(path, normalized_base_path).context("diff path")?;
            if path_to_string(&path)?.is_empty() {
                path = ".".into();
            }
            Ok(path)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flat_map(|path| {
            vec![path.clone()].into_iter().chain(
                extra_extensions
                    .iter()
                    .map(move |ext| with_extension(path.clone(), ext))
                    .filter(|path| base_path.join(path).exists()),
            )
        })
        .collect_vec())
}

fn with_extension(mut path: PathBuf, ext: &str) -> PathBuf {
    path.set_extension(ext);
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    /// Converts a descendant path into a path relative to the formatter root.
    #[test]
    fn prepares_a_relative_path() -> anyhow::Result<()> {
        let directory = tempdir()?;
        let source = directory.path().join("src").join("lib.rs");
        fs::create_dir_all(source.parent().unwrap())?;
        fs::write(&source, "")?;

        assert_eq!(
            prepare_paths(&[source], directory.path(), &[])?,
            vec![PathBuf::from("src/lib.rs")]
        );
        Ok(())
    }

    /// Uses the dot path when formatting the formatter root itself.
    #[test]
    fn prepares_the_base_path_as_dot() -> anyhow::Result<()> {
        let directory = tempdir()?;

        assert_eq!(
            prepare_paths(&[directory.path().to_owned()], directory.path(), &[])?,
            vec![PathBuf::from(".")]
        );
        Ok(())
    }

    /// Includes only existing sibling files with requested extensions.
    #[test]
    fn adds_existing_sibling_extensions() -> anyhow::Result<()> {
        let directory = tempdir()?;
        let stem = format!(
            "format-rust-sibling-{}",
            directory.path().file_name().unwrap().to_string_lossy()
        );
        let source = directory.path().join(format!("{stem}.rs"));
        fs::write(&source, "")?;
        fs::write(directory.path().join(format!("{stem}.dart")), "")?;

        assert_eq!(
            prepare_paths(&[source], directory.path(), &["dart", "swift"])?,
            vec![
                PathBuf::from(format!("{stem}.rs")),
                PathBuf::from(format!("{stem}.dart"))
            ]
        );
        Ok(())
    }

    /// Replaces a path extension without changing its parent directory.
    #[test]
    fn replaces_a_path_extension() {
        assert_eq!(
            with_extension(PathBuf::from("nested/api.rs"), "dart"),
            PathBuf::from("nested/api.dart")
        );
    }

    /// Rejects a path that cannot be represented as UTF-8.
    #[cfg(unix)]
    #[test]
    fn rejects_a_non_utf8_path() {
        use std::os::unix::ffi::OsStringExt;

        let path = PathBuf::from(std::ffi::OsString::from_vec(vec![0xff]));
        assert!(prepare_paths(&[path], Path::new("."), &[]).is_err());
    }
}
