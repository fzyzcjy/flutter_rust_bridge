use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context};

pub fn canonicalize_path<P: AsRef<Path>>(raw_path: P, base_dir: &Path) -> PathBuf {
    base_dir.join(raw_path)
}

pub fn glob_path(raw_path: &str, base_dir: &Path) -> Vec<PathBuf> {
    todo!()
}

pub fn path_to_string(path: &Path) -> anyhow::Result<String> {
    Ok(path.to_str().context("cannot convert path to str")?.to_owned())
}

pub fn find_parent_dir_with_file(path_start: &Path, probe_file_name: &str) -> anyhow::Result<PathBuf> {
    let mut path = path_start.to_owned();
    loop {
        if path.join(probe_file_name).is_file() { return Ok(path); }
        if !path.pop() { break; }
    }
    Err(anyhow!("Root of Dart library could not be inferred from Dart output"))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::utils::path_utils::canonicalize_path;

    #[test]
    fn test_canonicalize_path_simple() {
        // relative
        assert_eq!(canonicalize_path("./a.rs", &PathBuf::from("/x/y")), PathBuf::from("/x/y/a.rs"));

        // absolute
        assert_eq!(canonicalize_path("/a/b/c.rs", &PathBuf::from("/x/y")), PathBuf::from("/a/b/c.rs"));
    }

    #[test]
    fn test_glob_path_simple() {
        todo!()
    }
}
