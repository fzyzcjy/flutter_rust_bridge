use crate::command_run;
use crate::commands::command_runner::call_shell;
use crate::library::commands::command_runner::check_exit_code;
use log::info;
use std::path::Path;

#[allow(clippy::vec_init_then_push)]
pub fn flutter_create(name: &str, org: &Option<String>) -> anyhow::Result<()> {
    match org {
        Some(o) => {
            info!("Execute `flutter create  --org {o} {name}`");
            check_exit_code(
                &command_run!(call_shell[None, None], "flutter", "create", "--org", o, name )?,
            )
        }
        None => {
            info!("Execute `flutter create {name}`");
            check_exit_code(&command_run!(call_shell[None, None], "flutter", "create", name )?)
        }
    }
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

#[allow(clippy::vec_init_then_push)]
pub fn flutter_pub_get(path: &Path) -> anyhow::Result<()> {
    info!("Execute `flutter pub get` inside {path:?}");
    check_exit_code(&command_run!(call_shell[Some(path), None], "flutter", "pub", "get")?)
}
