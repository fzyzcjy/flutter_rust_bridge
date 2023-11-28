use crate::integration::integrator;
use crate::library::commands::flutter_create::flutter_create;
use log::debug;
use std::env;

/// Create a new Flutter + Rust project.
pub fn create(name: &str) -> anyhow::Result<()> {
    debug!("create name={name}");
    flutter_create(name)?;
    env::set_current_dir(env::current_dir()?.join(name))?;
    integrator::integrate()
}
