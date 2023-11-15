use std::path::{Path, PathBuf};
use log::debug;
use crate::library::commands::cargo_expand::cargo_expand;

pub(crate) fn read_rust_file(rust_file_path: &Path, rust_crate_dir: &Path) -> String {
    let (dir, module) = get_rust_mod(path);
    debug!("read_rust_file path={path:?} => dir={dir:?} module={module:?}");
    cargo_expand(&dir, module, path)
}

fn get_rust_mod(rust_file_path: &Path, rust_crate_dir: &Path) -> String {
    const SRC: &str = "/src/";

    #[cfg(windows)]
        let path = &path.replace('\\', "/");
    let src_index = rust_file_path.rfind(SRC).expect("src dir must exist in rust project");

    let dir = &rust_file_path[..src_index];
    #[cfg(windows)]
        let dir = dir.strip_prefix("//?/").unwrap_or(dir);

    let module = &rust_file_path[src_index + SRC.len()..];
    let module = module.strip_suffix("mod.rs").unwrap_or(module);
    let module = match module {
        "lib.rs" => None,
        "" => None,
        _ => {
            let module = module.replace('/', "::");
            Some(module.strip_suffix(".rs").map(String::from).unwrap_or(module))
        }
    };
    (String::from(dir), module)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_dir_and_mod_simple_mod() {
        let actual = get_rust_mod("/project/src/api.rs".into(), "/project".into());
        assert_eq!(Some("api".to_owned()), actual);
    }

    #[test]
    pub fn test_get_dir_and_mod_sub_mod() {
        let actual = get_rust_mod("/project/src/sub/subsub.rs".into(), "/project".into());
        assert_eq!(Some("sub::subsub".to_owned()), actual);
    }

    #[test]
    pub fn test_get_dir_and_mod_lib_rs() {
        let actual = get_rust_mod("/project/src/lib.rs".into(), "/project".into());
        assert_eq!(None, actual);
    }

    #[test]
    pub fn test_get_dir_and_mod_mod_rs() {
        let actual = get_rust_mod("/project/src/hello/mod.rs".into(), "/project".into());
        assert_eq!(Some("hello".to_owned()), actual);
    }
}
