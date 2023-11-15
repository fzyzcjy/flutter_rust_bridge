use std::path::{Path, PathBuf};
use crate::library::commands::cargo_expand::cargo_expand;

pub(crate) fn read_rust_file(path: &Path) -> String {
    let (dir, module) = get_dir_and_mod(path);
    cargo_expand(&dir, module, path)
}

fn get_dir_and_mod(path: &Path) -> (PathBuf, Option<String>) {
    const SRC: &str = "/src/";

    #[cfg(windows)]
        let path = &path.replace('\\', "/");
    let src_index = path.rfind(SRC).expect("src dir must exist in rust project");

    let dir = &path[..src_index];
    #[cfg(windows)]
        let dir = dir.strip_prefix("//?/").unwrap_or(dir);

    let module = &path[src_index + SRC.len()..];
    let module = module.strip_suffix("mod.rs").unwrap_or(module);
    let module = match module {
        "lib.rs" => None,
        "" => None,
        _ => {
            let module = module.replace('/', "::");
            Some(
                module
                    .strip_suffix(".rs")
                    .map(String::from)
                    .unwrap_or(module),
            )
        }
    };
    (String::from(dir), module)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_dir_and_mod_simple_mod() {
        let (dir, module) = get_dir_and_mod("/project/src/api.rs".into());
        assert_eq!("/project".into(), dir);
        assert_eq!(Some("api".to_owned()), module);
    }

    #[test]
    pub fn test_get_dir_and_mod_sub_mod() {
        let (dir, module) = get_dir_and_mod("/project/src/sub/subsub.rs".into());
        assert_eq!("/project".into(), dir);
        assert_eq!(Some("sub::subsub".to_owned()), module);
    }

    #[test]
    pub fn test_get_dir_and_mod_lib_rs() {
        let (dir, module) = get_dir_and_mod("/project/src/lib.rs".into());
        assert_eq!("/project".into(), dir);
        assert_eq!(None, module);
    }

    #[test]
    pub fn test_get_dir_and_mod_mod_rs() {
        let (dir, module) = get_dir_and_mod("/project/src/hello/mod.rs".into());
        assert_eq!("/project".into(), dir);
        assert_eq!(Some("hello".to_owned()), module);
    }
}
