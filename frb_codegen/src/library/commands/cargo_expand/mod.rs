mod pseudo;
mod real;

use crate::codegen::dumper::Dumper;
use crate::codegen::ConfigDumpContent;
use crate::command_args;
use crate::library::commands::command_runner::execute_command;
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{debug, info, warn};
use regex::Regex;
use std::borrow::Cow;
use std::env;
use std::path::Path;

pub(crate) fn run_cargo_expand(rust_crate_dir: &Path, dumper: &Dumper) -> Result<String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    debug!("run_cargo_expand manifest_dir={manifest_dir} rust_crate_dir={rust_crate_dir:?}");

    if !manifest_dir.is_empty()
        && normalize_windows_unc_path(&path_to_string(rust_crate_dir)?)
            == normalize_windows_unc_path(&manifest_dir)
    {
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

    let ans = real::run(rust_crate_dir)?;
    dumper.dump_str(ConfigDumpContent::Source, "cargo_expand.rs", &ans)?;
    Ok(ans)
}

