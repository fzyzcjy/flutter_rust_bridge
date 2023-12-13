use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::utils::console::simple_progress;
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use log::debug;
use std::path::PathBuf;

#[allow(clippy::vec_init_then_push)]
pub fn format_dart(path: &[PathBuf], line_length: u32) -> anyhow::Result<()> {
    let _pb = simple_progress("Run Dart formatter".to_owned(), 1);

    let path = normalize_windows_unc_paths(path)?;
    debug!("execute format_dart path={path:?} line_length={line_length}");

    let res = command_run!(
        call_shell[None],
        "dart",
        "format",
        "--line-length",
        line_length.to_string(),
        *path
    )?;
    check_exit_code(&res)?;
    Ok(())
}

pub(super) fn normalize_windows_unc_paths(paths: &[PathBuf]) -> anyhow::Result<Vec<PathBuf>> {
    (paths.iter())
        .map(|p| {
            Ok(normalize_windows_unc_path(&path_to_string(p)?)
                .to_owned()
                .into())
        })
        .collect()
}
