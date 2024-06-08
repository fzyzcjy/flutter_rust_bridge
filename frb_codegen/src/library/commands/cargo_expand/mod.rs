mod pseudo;
mod real;

use crate::codegen::dumper::Dumper;
use crate::utils::crate_name::CrateName;
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::Result;
use log::debug;
use std::env;
use std::path::Path;

pub(crate) fn run_cargo_expand(
    rust_crate_dir: &Path,
    interest_crate_name: Option<&CrateName>,
    dumper: &Dumper,
) -> Result<syn::File> {
    if can_execute_real(rust_crate_dir)? {
        real::run(rust_crate_dir, interest_crate_name, dumper)
    } else {
        pseudo::run(rust_crate_dir, interest_crate_name)
    }
}

fn can_execute_real(rust_crate_dir: &Path) -> anyhow::Result<bool> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    debug!("run_cargo_expand manifest_dir={manifest_dir} rust_crate_dir={rust_crate_dir:?}");
    Ok(manifest_dir.is_empty()
        || normalize_windows_unc_path(&path_to_string(rust_crate_dir)?)
            != normalize_windows_unc_path(&manifest_dir))
}
