use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn cargo_add(args: &[String], pwd: &Path) -> anyhow::Result<()> {
    check_exit_code(&command_run!(
        call_shell[Some(pwd), None],
        "cargo",
        "add",
        *args,
    )?)
}
