use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::library::commands::dart_fix::prepare_paths;
use log::debug;
use std::path::{Path, PathBuf};

#[allow(clippy::vec_init_then_push)]
pub fn format_dart(
    paths: &[PathBuf],
    base_path: &Path,
    line_length: u32,
    extra_extensions: &[&str],
) -> anyhow::Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    let paths = prepare_paths(paths, base_path, extra_extensions)?;
    debug!("execute format_dart paths={paths:?} line_length={line_length}");

    let res = command_run!(
        call_shell[Some(base_path), None],
        "dart",
        "format",
        "--line-length",
        line_length.to_string(),
        *paths
    )?;
    check_exit_code(&res)?;
    Ok(())
}
