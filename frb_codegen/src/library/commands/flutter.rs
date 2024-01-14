use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::{check_exit_code, ShellMode};
use log::info;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn flutter_create(
    name: &str,
    org: &Option<String>,
    shell_mode: Option<ShellMode>,
) -> anyhow::Result<()> {
    let mut args = vec![name];
    if let Some(o) = org {
        args.extend(["--org", o]);
    }
    info!(
        "Execute `flutter create {}` (this may take a while)",
        args.join(" ")
    );
    check_exit_code(&command_run!(call_shell[shell_mode, None, None], "flutter", "create", *args)?)
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_add(items: &[String], shell_mode: Option<ShellMode>) -> anyhow::Result<()> {
    info!(
        "Execute flutter pub add {} (this may take a while)",
        items.join(" ")
    );
    check_exit_code(&command_run!(
        call_shell[shell_mode, None, None],
        "flutter",
        "pub",
        "add",
        *items
    )?)
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_get(path: &Path, shell_mode: Option<ShellMode>) -> anyhow::Result<()> {
    info!("Execute `flutter pub get` inside {path:?} (this may take a while)");
    check_exit_code(
        &command_run!(call_shell[shell_mode, Some(path), None], "flutter", "pub", "get")?,
    )
}
