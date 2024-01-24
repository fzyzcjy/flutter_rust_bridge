use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::utils::path_utils::{normalize_windows_unc_path, path_to_string};
use anyhow::Context;
use log::debug;
use pathdiff::diff_paths;
use std::path::{Path, PathBuf};

#[allow(clippy::vec_init_then_push)]
pub fn format_dart(paths: &[PathBuf], base_path: &Path, line_length: u32) -> anyhow::Result<()> {
    let paths = prepare_paths(paths, base_path)?;
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

pub(super) fn prepare_paths(paths: &[PathBuf], base_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let normalized_base_path = normalize_windows_unc_path(&path_to_string(base_path)?);
    (paths.iter())
        .map(|path| {
            let mut path: PathBuf =
                (normalize_windows_unc_path(&path_to_string(path)?).to_owned()).into();
            path = diff_paths(path, normalized_base_path).context("diff path")?;
            if path_to_string(&path)?.is_empty() {
                path = ".".into();
            }
            Ok(path)
        })
        .collect()
}
