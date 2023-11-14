use std::path::{Path, PathBuf};

pub fn canonicalize_path(raw_path: &str, base_dir: &Path) -> PathBuf {
    todo!()
}

pub fn glob_path(raw_path: &str, base_dir: &Path) -> Vec<PathBuf> {
    todo!()
}

pub fn path_to_string(path: &Path) -> anyhow::Result<String> {
    Ok(path.into_os_string().into_string()?)
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
