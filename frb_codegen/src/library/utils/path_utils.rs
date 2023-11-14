use std::path::{Path, PathBuf};
use anyhow::anyhow;

pub fn canonicalize_path<P: AsRef<Path>>(raw_path: P, base_dir: &Path) -> PathBuf {
    todo!()
}

pub fn glob_path(raw_path: &str, base_dir: &Path) -> Vec<PathBuf> {
    todo!()
}

pub fn path_to_string(path: &Path) -> anyhow::Result<String> {
    Ok(path.into_os_string().into_string()?)
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
    use crate::utils::path_utils::canonicalize_path;

    #[test]
    fn test_canonicalize_path_simple() {
        // relative
        assert_eq!(canonicalize_path("./a.rs", "/x/y".into()), "/x/y/a.rs");

        // absolute
        assert_eq!(canonicalize_path("/a/b/c.rs", "/x/y".into()), "/a/b/c.rs");
    }
}
