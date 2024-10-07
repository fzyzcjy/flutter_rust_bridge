use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use anyhow::Result;
use log::debug;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn dart_fix(output_path: &Path) -> Result<()> {
    debug!("execute dart_fix output_path={output_path:?}");

    let res = command_run!(
        call_shell[Some(output_path), None],
        "dart",
        "fix",
        "--apply",
        "."
    )?;
    check_exit_code(&res)?;
    Ok(())
}
