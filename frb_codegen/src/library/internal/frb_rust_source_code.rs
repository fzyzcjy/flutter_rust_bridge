use std::fs;
use std::path::Path;

pub(crate) fn generate_frb_rust_source_code(repo_base_dir: &Path) -> anyhow::Result<()> {
    let path_target = repo_base_dir
        .join("frb_rust")
        .join("src")
        .join("internal_generated")
        .join("mod.rs");

    let text = TODO;

    fs::write(path_target, text)?;
    Ok(())
}
