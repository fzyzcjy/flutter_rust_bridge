use crate::integration::integrator;
use crate::integration::integrator::IntegrateConfig;
use crate::library::commands::flutter::flutter_create;
use anyhow::ensure;
use log::{debug, info};
use std::path::Path;
use std::{env, fs};

pub struct CreateConfig {
    pub name: String,
    pub org: Option<String>,
    pub enable_local_dependency: bool,
    pub rust_crate_name: Option<String>,
    pub rust_crate_dir: String,
}

/// Create a new Flutter + Rust project.
pub fn create(config: CreateConfig) -> anyhow::Result<()> {
    let dart_root = env::current_dir()?.join(&config.name);
    debug!(
        "create name={} org={:?} dart_root={dart_root:?}",
        config.name, config.org
    );

    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    ensure!(
        !dart_root.exists(),
        "The target folder {:?} already exists. Please use the `integrate` command in this case",
        dart_root,
    );
    // frb-coverage:ignore-end

    flutter_create(&config.name, &config.org)?;

    env::set_current_dir(&dart_root)?;

    remove_unnecessary_files(&dart_root)?;

    info!("Step: Inject flutter_rust_bridge related code");
    integrator::integrate(IntegrateConfig {
        enable_integration_test: true,
        enable_local_dependency: config.enable_local_dependency,
        rust_crate_name: config.rust_crate_name,
        rust_crate_dir: config.rust_crate_dir,
    })
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn remove_unnecessary_files(dart_root: &Path) -> anyhow::Result<()> {
    // frb-coverage:ignore-end
    fs::remove_file(dart_root.join("test").join("widget_test.dart"))?;
    fs::remove_file(dart_root.join("lib").join("main.dart"))?;
    Ok(())
}
