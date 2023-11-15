use std::path::Path;
use crate::library::commands::cargo_expand::cargo_expand;

pub(crate) fn read_rust_file(path: &Path) -> String {
    let path = path.to_str().unwrap();
    let (dir, module) = get_dir_and_mod(path);
    cargo_expand(&dir, module, path)
}

// TODO refactor
fn get_dir_and_mod(path: &str) -> (String, Option<String>) {
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
