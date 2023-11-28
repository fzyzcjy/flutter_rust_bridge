use crate::library::commands::flutter_create::flutter_create;
use log::debug;

/// Create a new Flutter + Rust project.
pub fn create(name: &str) -> anyhow::Result<()> {
    debug!("create name={name}");

    flutter_create(name)?;

    todo!("create")
}
