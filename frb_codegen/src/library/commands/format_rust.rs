use crate::command_run;
use crate::library::commands::command_runner::{call_shell, check_exit_code};
use crate::library::commands::format_dart::prepare_paths;
use log::debug;
use std::path::{Path, PathBuf};

pub fn format_rust(paths: &[PathBuf], base_path: &Path) -> anyhow::Result<()> {
    let paths = prepare_paths(paths, base_path, &[])?;
    debug!("execute format_rust paths={paths:?}");

    check_exit_code(&command_run!(
        call_shell[Some(base_path), None],
        "rustfmt",
        // otherwise cannot understand `async move`
        "--edition",
        "2018",
        *paths
    )?)
}
