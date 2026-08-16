//! Guard the assets that get embedded into the binary at compile time.
//!
//! `assets/integration_template/cargokit/**/cargokit` are git submodules, and
//! `include_dir!` accepts an empty directory without complaining: the build
//! succeeds and silently produces a binary whose templates are missing the
//! cargokit build scripts. `frb create` then emits a project that cannot build.
//! Fail here instead, while the cause is still obvious.

use std::path::{Path, PathBuf};

/// Directories that must carry real content, each with a file that proves it.
/// The sentinel is what cargokit actually needs at generation time, so its
/// absence is exactly the breakage we want to catch.
const REQUIRED_ASSET_DIRS: [(&str, &str); 2] = [
    (
        "assets/integration_template/cargokit/app/rust_builder/cargokit",
        "run_build_tool.sh",
    ),
    (
        "assets/integration_template/cargokit/plugin/cargokit",
        "run_build_tool.sh",
    ),
];

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // `include_dir!` bakes the contents in at compile time, so without this the
    // crate is not rebuilt after the templates change and the binary keeps
    // shipping stale assets.
    println!("cargo:rerun-if-changed=assets");

    let missing = (REQUIRED_ASSET_DIRS.iter())
        .filter(|(dir, sentinel)| !is_populated(&manifest_dir.join(dir), sentinel))
        .map(|(dir, _)| *dir)
        .collect::<Vec<_>>();

    if !missing.is_empty() {
        panic!("{}", error_message(&missing));
    }
}

fn is_populated(dir: &Path, sentinel: &str) -> bool {
    dir.join(sentinel).is_file()
}

fn error_message(missing: &[&str]) -> String {
    format!(
        "\n\n\
         Cargokit git submodules are missing or empty:\n\
         {}\n\n\
         These are embedded into the binary at compile time, so building now would\n\
         produce a `flutter_rust_bridge_codegen` whose `create`/`integrate` output\n\
         is missing the cargokit build scripts.\n\n\
         To fix, run at the repository root:\n\
         \x20   git submodule update --init --recursive\n\n\
         (If you are building from a crates.io release rather than a git checkout,\n\
         this indicates a packaging problem; please report it.)\n",
        missing
            .iter()
            .map(|dir| format!("  - {dir}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
