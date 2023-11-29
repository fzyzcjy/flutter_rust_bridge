use crate::library::commands::command_runner::{check_exit_code, execute_command};
use crate::library::commands::format_dart::normalize_windows_unc_paths;
use log::debug;
use std::path::PathBuf;

pub fn format_rust(path: &[PathBuf]) -> anyhow::Result<()> {
    let path = normalize_windows_unc_paths(path)?;
    debug!("execute format_rust path={path:?}");
    check_exit_code(&execute_command("rustfmt", &path, None)?)
}
