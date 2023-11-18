use crate::library::commands::command_runner::execute_command;
use anyhow::bail;
use log::debug;
use std::path::PathBuf;

pub fn format_rust(path: &[PathBuf]) -> anyhow::Result<()> {
    debug!("execute format_rust path={path:?}");
    let res = execute_command("rustfmt", path, None)?;
    if !res.status.success() {
        bail!(
            "Rust formatting failed: {}",
            String::from_utf8_lossy(&res.stderr),
        )
    }
    Ok(())
}
