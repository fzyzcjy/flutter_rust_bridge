use log::warn;
use std::path::Path;

pub(super) fn run(rust_crate_dir: &Path) -> anyhow::Result<String> {
    warn!(
        "Skip cargo-expand on {rust_crate_dir:?}, \
         because cargo is already running and would block cargo-expand. \
         This might cause errors if your api contains macros or complex mods."
    );

    TODO
}
