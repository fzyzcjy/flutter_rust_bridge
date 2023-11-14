use std::path::{Path, PathBuf};

fn canonicalize_path(raw_path: &str, base_dir: &Path) -> PathBuf {
    todo!()
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
