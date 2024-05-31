use crate::codegen::Config;
use std::path::PathBuf;

pub(super) struct ConfigRustRootAndRustInput {
    rust_root: PathBuf,
    rust_input: String,
}

pub(super) fn migrate_rust_input_config(config: &Config) -> ConfigRustRootAndRustInput {
    ConfigRustRootAndRustInput {
        rust_root: TODO,
        rust_input: TODO,
    }
}
