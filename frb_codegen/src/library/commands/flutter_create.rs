use crate::command_run;
use crate::commands::command_runner::call_shell;
use anyhow::bail;
use log::info;

#[allow(clippy::vec_init_then_push)]
pub fn flutter_create(name: &str) -> anyhow::Result<()> {
    info!("Execute `flutter create {name}`");

    let res = command_run!(call_shell[None], "flutter", "create", name,)?;
    if !res.status.success() {
        let msg = String::from_utf8_lossy(&res.stderr);
        bail!("`flutter create` failed: {msg}");
    }
    Ok(())
}
