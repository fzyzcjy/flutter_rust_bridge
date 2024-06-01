pub(super) fn run(rust_crate_dir: &Path) -> Result<String> {
    // We do not care about this warning message
    // frb-coverage:ignore-start
    warn!(
        "Skip cargo-expand on {rust_crate_dir:?}, \
             because cargo is already running and would block cargo-expand. \
             This might cause errors if your api contains macros or complex mods."
    );
    // frb-coverage:ignore-end
    todo!("Usage in build.rs for new mod-based system is not implemented yet. Feel free to create an issue if you need this!")
    // return Ok(fs::read_to_string(rust_file_path)?);
}
