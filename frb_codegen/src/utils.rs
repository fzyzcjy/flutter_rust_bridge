use std::fs;
use std::path::PathBuf;

pub fn path_stem(path: &str) -> String {
    PathBuf::from(path)
        .file_stem()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}

pub fn path_filename(path: &str) -> String {
    PathBuf::from(path)
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}

pub fn with_changed_file<F: FnOnce()>(path: &str, append_content: &str, f: F) {
    let content_original = fs::read_to_string(&path).unwrap();
    fs::write(&path, content_original.clone() + append_content).unwrap();

    f();

    fs::write(&path, content_original).unwrap();
}
