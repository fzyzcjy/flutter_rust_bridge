use log::warn;
use std::fs;
use std::path::Path;

pub(super) fn run(rust_crate_dir: &Path) -> anyhow::Result<syn::File> {
    warn!(
        "Skip cargo-expand on {rust_crate_dir:?}, \
         because cargo is already running and would block cargo-expand. \
         This might cause errors if your api contains macros or complex mods."
    );

    parse_file(&rust_crate_dir.join("src/lib.rs"))
}

fn parse_file(path: &Path) -> anyhow::Result<syn::File> {
    let code = fs::read_to_string(&path)?;
    let ast = syn::parse_file(&code)?;
    TODO
}
