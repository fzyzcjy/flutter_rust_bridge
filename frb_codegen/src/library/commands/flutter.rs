use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use log::info;

#[allow(clippy::vec_init_then_push)]
pub fn flutter_create(name: &str) -> anyhow::Result<()> {
    info!("Execute `flutter create {name}`");
    check_exit_code(&command_run!(call_shell[None, None], "flutter", "create", name)?)
}

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_add(items: &[String]) -> anyhow::Result<()> {
    info!("Execute flutter pub add {items:?}");
    check_exit_code(&command_run!(
        call_shell[None, None],
        "flutter",
        "pub",
        "add",
        *items
    )?)
}
