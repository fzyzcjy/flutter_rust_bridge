use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

// pub(crate) fn glob_path(pattern: &Path) -> Result<Vec<PathBuf>> {
//     let pattern = normalize_windows_unc_path(pattern.to_str().context("cannot convert to str")?);
//     glob::glob(pattern)?
//         .filter_map(Result::ok)
//         .map(|p| canonicalize_with_error_message(&p))
//         .collect::<Result<Vec<_>>>()
// }

pub(crate) fn path_to_string(path: &Path) -> Result<String> {
    Ok(path
        .to_str()
        .context("cannot convert path to str")?
        .to_owned())
}

pub(crate) fn find_parent_dir_with_file(
    path_start: &Path,
    probe_file_name: &str,
) -> Option<PathBuf> {
    let mut path = path_start.to_owned();
    loop {
        if path.join(probe_file_name).is_file() {
            return Some(path);
        }
        if !path.pop() {
            break;
        }
    }
    None
}

pub(crate) fn find_dart_package_dir(dart_file_path: &Path) -> Result<PathBuf> {
    find_parent_dir_with_file(dart_file_path, "pubspec.yaml").with_context(|| {
        // frb-coverage:ignore-start
        // This will stop the whole generator and tell the users, so we do not care about testing it
        format!("Fail to detect dart package from dart_file_path={dart_file_path:?}")
        // frb-coverage:ignore-end
    })
}

// pub(crate) fn find_rust_crate_dir(rust_file_path: &Path) -> Result<PathBuf> {
//     find_parent_dir_with_file(rust_file_path, "Cargo.toml").with_context(|| {
//         // frb-coverage:ignore-start
//         // This will stop the whole generator and tell the users, so we do not care about testing it
//         format!("Fail to detect rust crate dir from rust_file_path={rust_file_path:?}")
//         // frb-coverage:ignore-end
//     })
// }

pub(crate) fn normalize_windows_unc_path(path: &str) -> &str {
    // on windows get rid of the UNC path
    path.strip_prefix(r"\\?\").unwrap_or(path)
}

pub(crate) fn canonicalize_with_error_message(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("Fail to canonicalize path={path:?}"))
}

#[cfg(test)]
mod tests {
    use crate::utils::path_utils::{
        canonicalize_with_error_message, find_dart_package_dir, find_parent_dir_with_file,
        normalize_windows_unc_path, path_to_string,
    };
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    /// Returns None when no ancestor contains the probe file.
    #[test]
    fn test_find_parent_dir_with_file_failure_case() {
        assert_eq!(
            find_parent_dir_with_file(&PathBuf::new(), "whatever_not_exist_file"),
            None
        );
    }

    #[allow(clippy::join_absolute_paths)]
    /// Preserves PathBuf joining semantics used by the disabled glob helper.
    #[test]
    fn test_glob_path_simple() -> Result<()> {
        let temp_dir = tempdir()?;
        fs::write(temp_dir.path().join("apple.rs"), "")?;
        fs::write(temp_dir.path().join("orange.rs"), "")?;
        fs::write(temp_dir.path().join("aha.rs"), "")?;

        // fn extract_names(items: &[PathBuf]) -> HashSet<String> {
        //     items
        //         .iter()
        //         .map(|x| x.file_name().unwrap().to_str().unwrap().to_owned())
        //         .collect()
        // }

        assert_eq!(
            PathBuf::from("/a/b").join("*.rs"),
            PathBuf::from("/a/b/*.rs")
        );
        assert_eq!(
            PathBuf::from("/a/b").join("c/*.rs"),
            PathBuf::from("/a/b/c/*.rs")
        );
        assert_eq!(
            PathBuf::from("/a/b").join("/c/*.rs"),
            PathBuf::from("/c/*.rs")
        );

        // assert_eq!(
        //     extract_names(&glob_path(&temp_dir.path().join("*.rs"))?),
        //     vec![
        //         "apple.rs".to_owned(),
        //         "orange.rs".to_owned(),
        //         "aha.rs".to_owned()
        //     ]
        //     .into_iter()
        //     .collect(),
        // );
        //
        // assert_eq!(
        //     extract_names(&glob_path(&temp_dir.path().join("a*.rs"))?),
        //     vec!["apple.rs".to_owned(), "aha.rs".to_owned()]
        //         .into_iter()
        //         .collect(),
        // );

        Ok(())
    }

    /// Finds the closest ancestor containing the requested marker file.
    #[test]
    fn test_find_parent_dir_with_file_returns_nearest_matching_ancestor() -> Result<()> {
        let temp_dir = tempdir()?;
        let package_dir = temp_dir.path().join("package");
        let nested_dir = package_dir.join("lib/src");
        fs::create_dir_all(&nested_dir)?;
        fs::write(package_dir.join("pubspec.yaml"), "name: package")?;
        fs::write(temp_dir.path().join("pubspec.yaml"), "name: outer")?;

        assert_eq!(
            find_parent_dir_with_file(&nested_dir, "pubspec.yaml"),
            Some(package_dir),
        );
        Ok(())
    }

    /// Finds Dart package roots from files at different nesting levels.
    #[test]
    fn test_find_dart_package_dir_finds_immediate_and_nested_pubspec() -> Result<()> {
        let temp_dir = tempdir()?;
        let package_dir = temp_dir.path().join("package");
        let nested_file = package_dir.join("lib/src/api.dart");
        fs::create_dir_all(nested_file.parent().unwrap())?;
        fs::write(package_dir.join("pubspec.yaml"), "name: package")?;

        assert_eq!(
            find_dart_package_dir(&package_dir.join("main.dart"))?,
            package_dir
        );
        assert_eq!(find_dart_package_dir(&nested_file)?, package_dir);
        Ok(())
    }

    /// Returns a contextual error when no Dart package marker exists.
    #[test]
    fn test_find_dart_package_dir_returns_contextual_error_when_absent() -> Result<()> {
        let temp_dir = tempdir()?;
        let error = find_dart_package_dir(&temp_dir.path().join("lib/api.dart")).unwrap_err();

        assert!(error.to_string().contains("Fail to detect dart package"));
        Ok(())
    }

    /// Converts UTF-8 paths and removes only the Windows UNC prefix.
    #[test]
    fn test_path_to_string_and_normalize_windows_unc_path() -> Result<()> {
        assert_eq!(path_to_string(PathBuf::from("a/é").as_path())?, "a/é");
        assert_eq!(normalize_windows_unc_path(r"\\?\C:\work"), r"C:\work");
        assert_eq!(
            normalize_windows_unc_path(r"\\server\share"),
            r"\\server\share"
        );
        assert_eq!(normalize_windows_unc_path("ordinary/path"), "ordinary/path");
        Ok(())
    }

    /// Canonicalizes existing paths and contextualizes missing-path errors.
    #[test]
    fn test_canonicalize_with_error_message_handles_existing_and_missing_paths() -> Result<()> {
        let temp_dir = tempdir()?;
        let existing_path = temp_dir.path().join("existing.txt");
        fs::write(&existing_path, "contents")?;

        assert_eq!(
            canonicalize_with_error_message(&existing_path)?,
            existing_path.canonicalize()?
        );

        let missing_path = temp_dir.path().join("missing.txt");
        let error = canonicalize_with_error_message(&missing_path).unwrap_err();
        assert!(error.to_string().contains("Fail to canonicalize path="));
        Ok(())
    }
}
