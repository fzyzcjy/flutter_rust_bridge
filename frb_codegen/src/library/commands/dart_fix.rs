use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use crate::library::commands::fvm::command_arg_maybe_fvm;
use anyhow::Result;
use log::debug;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn dart_fix(base_path: &Path) -> Result<()> {
    debug!("execute dart_fix base_path={base_path:?}");

    let res = command_run!(
        call_shell[Some(base_path), None],
        ?command_arg_maybe_fvm(Some(base_path)),
        "dart",
        "fix",
        "--apply",
        "."
    )?;
    check_exit_code(&res)?;
    Ok(())
}
