use crate::integration::integrator;
use crate::library::commands::flutter::flutter_create;
use log::{debug, info};
use std::path::Path;
use std::{env, fs};

/// Create a new Flutter + Rust project.
pub fn create(name: &str, enable_local_dependency: bool) -> anyhow::Result<()> {
    debug!("create name={name}");

    flutter_create(name)?;

    let dart_root = env::current_dir()?.join(name);
    env::set_current_dir(&dart_root)?;

    remove_unnecessary_files(&dart_root)?;

    info!("Step: Inject flutter_rust_bridge related code");
    integrator::integrate(true, enable_local_dependency)
}

fn remove_unnecessary_files(dart_root: &Path) -> anyhow::Result<()> {
    fs::remove_file(dart_root.join("test").join("widget_test.dart"))?;
    fs::remove_file(dart_root.join("lib").join("main.dart"))?;
    Ok(())
}
