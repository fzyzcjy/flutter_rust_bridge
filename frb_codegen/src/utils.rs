use std::fs;
use std::path::Path;

pub fn mod_from_rust_path(code_path: &str, crate_path: &str) -> String {
    Path::new(code_path)
        .strip_prefix(Path::new(crate_path).join("src"))
        .unwrap()
        .with_extension("")
        .into_os_string()
        .into_string()
        .unwrap()
        .replace('/', "::")
}

pub fn with_changed_file<F: FnOnce()>(path: &str, append_content: &str, f: F) {
    let content_original = fs::read_to_string(&path).unwrap();
    fs::write(&path, content_original.clone() + append_content).unwrap();

    f();

    fs::write(&path, content_original).unwrap();
}
