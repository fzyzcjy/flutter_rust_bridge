use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::library::commands::dart_format::prepare_paths;
use anyhow::Result;
use log::debug;
use std::path::{Path, PathBuf};

#[allow(clippy::vec_init_then_push)]
pub fn dart_fix(paths: &[PathBuf], base_path: &Path, extra_extensions: &[&str]) -> Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    let paths = prepare_paths(paths, base_path, extra_extensions)?;
    debug!("execute dart_fix paths={paths:?}");

    let res = command_run!(
        call_shell[Some(base_path), None],
        "dart",
        "fix",
        "--apply",
        *paths
    )?;
    check_exit_code(&res)?;
    Ok(())
}
