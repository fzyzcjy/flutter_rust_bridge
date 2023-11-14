use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

pub fn glob_path(pattern: &Path) -> Result<Vec<PathBuf>> {
    let pattern = pattern.to_str().context("cannot convert to str")?;
    Ok(glob::glob(pattern)?.filter_map(Result::ok).collect())
}

pub fn path_to_string(path: &Path) -> Result<String> {
    Ok(path.to_str().context("cannot convert path to str")?.to_owned())
}

pub fn find_parent_dir_with_file(path_start: &Path, probe_file_name: &str) -> Option<PathBuf> {
    let mut path = path_start.to_owned();
    loop {
        if path.join(probe_file_name).is_file() { return Some(path); }
        if !path.pop() { break; }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use crate::utils::path_utils::glob_path;
    use anyhow::Result;

    #[test]
    fn test_glob_path_simple() -> Result<()> {
        let temp_dir = tempdir()?;
        fs::write(&temp_dir.path().join("apple.rs"), "")?;
        fs::write(&temp_dir.path().join("orange.rs"), "")?;
        fs::write(&temp_dir.path().join("aha.rs"), "")?;

        fn extract_names(items: &[PathBuf]) -> HashSet<String> {
            items.iter().map(|x| x.file_name().unwrap().to_str().unwrap().to_owned()).collect()
        }

        assert_eq!(PathBuf::from("/a/b").join("*.rs"), PathBuf::from("/a/b/*.rs"));
        assert_eq!(PathBuf::from("/a/b").join("c/*.rs"), PathBuf::from("/a/b/c/*.rs"));
        assert_eq!(PathBuf::from("/a/b").join("/c/*.rs"), PathBuf::from("/c/*.rs"));

        assert_eq!(
            extract_names(&glob_path(&temp_dir.path().join("*.rs"))?),
            vec!["apple.rs".to_owned(), "orange.rs".to_owned(), "aha.rs".to_owned()].into_iter().collect(),
        );

        assert_eq!(
            extract_names(&glob_path(&temp_dir.path().join("a*.rs"))?),
            vec!["apple.rs".to_owned(), "aha.rs".to_owned()].into_iter().collect(),
        );

        Ok(())
    }
}
