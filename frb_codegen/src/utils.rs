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

pub fn with_changed_file(
    path: &str,
    append_content: &str,
    generate_c_from_rust_func: impl FnOnce() -> anyhow::Result<()>,
    generate_dart_from_c_func: impl FnOnce() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    let content_original = fs::read_to_string(&path)?;
    fs::write(&path, content_original.clone() + append_content)?;

    generate_c_from_rust_func()?;
    generate_dart_from_c_func()?;

    Ok(fs::write(&path, content_original)?)
}
