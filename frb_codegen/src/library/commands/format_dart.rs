use crate::command_run;
use crate::commands::command_runner::call_shell;
use anyhow::bail;
use log::debug;
use std::path::PathBuf;

#[allow(clippy::vec_init_then_push)]
pub fn format_dart(path: &[PathBuf], line_length: u32) -> anyhow::Result<()> {
    debug!("execute format_dart path={path:?} line_length={line_length}");
    let res = command_run!(
        call_shell[None],
        "dart",
        "format",
        "--line-length",
        line_length.to_string(),
        *path
    )?;
    if !res.status.success() {
        bail!(
            "Dart formatting failed: {}",
            String::from_utf8_lossy(&res.stderr),
        )
    }
    Ok(())
}
