use crate::integration::integrator;
use crate::integration::integrator::IntegrateConfig;
use crate::library::commands::flutter::flutter_create;
use crate::misc::ProjectType;
use anyhow::{bail, ensure};
use log::{debug, info};
use std::path::Path;
use std::{env, fs};

pub struct CreateConfig {
    pub name: String,
    pub org: Option<String>,
    pub enable_local_dependency: bool,
    pub rust_crate_name: Option<String>,
    pub rust_crate_dir: String,
    pub template: ProjectType,
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

    flutter_create(&config.name, &config.org, &config.template)?;

    env::set_current_dir(&dart_root)?;

    remove_unnecessary_files(&dart_root)?;

    info!("Step: Inject flutter_rust_bridge related code");
    integrator::integrate(IntegrateConfig {
        enable_integration_test: true,
        enable_local_dependency: config.enable_local_dependency,
        rust_crate_name: config.rust_crate_name,
        rust_crate_dir: config.rust_crate_dir,
        template: config.template,
    })
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn remove_unnecessary_files(dart_root: &Path) -> anyhow::Result<()> {
    // frb-coverage:ignore-end
    let lib_dir = dart_root.join("lib");
    let test_dir = dart_root.join("test");

    fn remove_files_in_dir(dir: &Path) -> anyhow::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(&path)?;
            } else if path.is_dir() {
                bail!("An unexpected directory was encountered: {:?}", path);
            }
        }
        Ok(())
    }
    remove_files_in_dir(&lib_dir)?;
    remove_files_in_dir(&test_dir)?;

    Ok(())
}
