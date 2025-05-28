use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::library::commands::fvm::command_arg_maybe_fvm;
use anyhow::Result;
use log::debug;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn dart_format(base_path: &Path, line_length: u32) -> Result<()> {
    debug!("execute dart_format base_path={base_path:?} line_length={line_length}");

    let res = command_run!(
        call_shell[Some(base_path), None],
        ?command_arg_maybe_fvm(Some(base_path)),
        "dart",
        "format",
        "--line-length",
        line_length.to_string(),
        ".",
    )?;
    check_exit_code(&res)?;
    Ok(())
}
