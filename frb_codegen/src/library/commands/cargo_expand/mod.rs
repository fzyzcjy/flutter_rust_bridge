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
    let ans = if can_execute_real(rust_crate_dir)? {
        real::run(rust_crate_dir)?
    } else {
        pseudo::run(rust_crate_dir)?
    };

    dumper.dump_str(ConfigDumpContent::Source, "cargo_expand.rs", &ans)?;
    Ok(ans)
}

fn can_execute_real(rust_crate_dir: &Path) -> anyhow::Result<bool> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    debug!("run_cargo_expand manifest_dir={manifest_dir} rust_crate_dir={rust_crate_dir:?}");
    Ok(manifest_dir.is_empty()
        || normalize_windows_unc_path(&path_to_string(rust_crate_dir)?)
            != normalize_windows_unc_path(&manifest_dir))
}
