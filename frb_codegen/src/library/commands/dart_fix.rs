use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use anyhow::Result;
use log::debug;
use std::path::{Path, PathBuf};

#[allow(clippy::vec_init_then_push)]
pub fn dart_fix(paths: &[PathBuf], base_path: &Path, extra_extensions: &[&str]) -> Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    debug!("execute dart_fix");

    let res = command_run!(
        call_shell[Some(base_path), None],
        "dart",
        "fix",
        "--apply",
    )?;
    check_exit_code(&res)?;
    Ok(())
}
