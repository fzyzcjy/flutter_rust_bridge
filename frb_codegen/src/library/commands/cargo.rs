use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use anyhow::Result;
use log::warn;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn cargo_add(args: &[&str], pwd: &Path) -> anyhow::Result<()> {
    check_exit_code(&command_run!(
        call_shell[Some(pwd), None],
        "cargo",
        "add",
        *args,
    )?)
}

#[allow(clippy::vec_init_then_push)]
pub fn cargo_fetch(pwd: &Path) -> Result<()> {
    // frb-coverage:ignore-start
    let output = command_run!(
        call_shell[None, None],
        "cargo",
        "fetch",
        "--manifest-path",
        pwd.join("Cargo.toml"),
    )?;
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success()
        && stderr.contains("current package believes it's in a workspace when it's not")
    {
        warn!(
            "Skip `cargo fetch` because the manifest is nested under another workspace: {}",
            pwd.display()
        );
        return Ok(());
    }

    check_exit_code(&output)
    // frb-coverage:ignore-end
}
