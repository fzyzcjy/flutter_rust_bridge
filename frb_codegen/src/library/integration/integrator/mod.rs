mod backend_cargokit;
mod overlay;
mod pubspec;

use self::backend_cargokit::{
    exclude_cargokit_from_outer_analyzer, modify_permissions, setup_cargokit_dependencies,
};
use self::overlay::{compute_replacements, execute_overlay_templates};
pub(crate) use self::pubspec::pub_add_dependency_frb;
use self::pubspec::{add_publish_to_none, pub_add_dependencies};
use crate::library::commands::cargo::cargo_fetch;
use crate::library::commands::dart_fix::dart_fix;
use crate::library::commands::dart_format::dart_format;
use crate::library::commands::flutter::{platform_list_contains_ohos, resolve_flutter_platforms};
use crate::misc::{FvmInstallMode, IntegrationBackend, Template};
use crate::utils::dart_repository::get_dart_package_name;
use crate::utils::path_utils::find_dart_package_dir;
use anyhow::Result;
use log::{debug, info};
use std::env;
use std::path::Path;

const REFRESH_CARGO_LOCK_ORDERING_ENV_VAR: &str = "FRB_REFRESH_CARGO_LOCK_ORDERING";

pub struct IntegrateConfig {
    pub enable_write_lib: bool,
    pub enable_integration_test: bool,
    pub enable_dart_fix: bool,
    pub enable_dart_format: bool,
    pub enable_local_dependency: bool,
    pub rust_crate_name: Option<String>,
    pub rust_crate_dir: String,
    pub template: Template,
    pub integration_backend: IntegrationBackend,
    pub platforms: Option<String>,
    pub fvm_install_mode: FvmInstallMode,
}

/// Integrate Rust into existing Flutter project.
// ref: https://matejknopp.com/post/flutter_plugin_in_rust_with_no_prebuilt_binaries/
pub fn integrate(config: IntegrateConfig) -> Result<()> {
    let dart_root = find_dart_package_dir(&env::current_dir()?)?;
    debug!("integrate dart_root={dart_root:?}");
    let dart_package_name = get_dart_package_name(&dart_root)?;
    let rust_crate_name = config
        .rust_crate_name
        .clone()
        .unwrap_or(match &config.template {
            Template::App => {
                format!("rust_lib_{dart_package_name}")
            }
            Template::Plugin => dart_package_name.to_owned(),
        });
    let platforms = resolve_flutter_platforms(config.template, config.platforms.clone())?;
    let include_ohos = platform_list_contains_ohos(&platforms);

    info!("Overlay template onto project");
    let replacements =
        compute_replacements(&config, &dart_package_name, &rust_crate_name, include_ohos);
    execute_overlay_templates(
        &replacements,
        &dart_root,
        &config,
        include_ohos,
        &dart_package_name,
    )?;

    if config.enable_local_dependency && config.template == Template::Plugin {
        add_publish_to_none(&dart_root)?;
    }

    if config.integration_backend == IntegrationBackend::Cargokit {
        info!("Modify file permissions");
        modify_permissions(&dart_root, &config.template)?;
    }

    info!("Add pub dependencies");
    pub_add_dependencies(
        config.enable_integration_test,
        config.enable_local_dependency,
        &rust_crate_name,
        &config.template,
        config.integration_backend,
        config.fvm_install_mode,
    )?;

    if config.integration_backend == IntegrationBackend::Cargokit {
        info!("Setup cargokit dependencies");
        setup_cargokit_dependencies(&dart_root, &config.template, config.fvm_install_mode)?;

        exclude_cargokit_from_outer_analyzer(&dart_root, &config.template)?;
    }

    if config.enable_dart_fix {
        info!("Apply Dart fixes");
        dart_fix(&dart_root, config.fvm_install_mode)?;
    } else {
        info!("Dart fix is disabled.")
    }

    if config.enable_dart_format {
        info!("Format Dart code");
        dart_format(&dart_root, 80, config.fvm_install_mode)?;
    } else {
        info!("Dart format is disabled.");
    }

    maybe_refresh_cargo_lock_ordering(&dart_root, &config.rust_crate_dir)?;

    Ok(())
}

fn maybe_refresh_cargo_lock_ordering(dart_root: &Path, rust_crate_dir: &str) -> Result<()> {
    if !should_refresh_cargo_lock_ordering() {
        debug!(
            "Skip Cargo.lock ordering refresh; set {REFRESH_CARGO_LOCK_ORDERING_ENV_VAR}=1 to enable"
        );
        return Ok(());
    }

    refresh_cargo_lock_ordering(dart_root, rust_crate_dir)
}

fn should_refresh_cargo_lock_ordering() -> bool {
    env::var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR).unwrap_or_default() == "1"
}

fn refresh_cargo_lock_ordering(dart_root: &Path, rust_crate_dir: &str) -> Result<()> {
    info!("Refresh Cargo.lock ordering because {REFRESH_CARGO_LOCK_ORDERING_ENV_VAR}=1");
    cargo_fetch(&dart_root.join(rust_crate_dir))
}

#[cfg(test)]
mod tests {
    use super::{
        maybe_refresh_cargo_lock_ordering, refresh_cargo_lock_ordering,
        should_refresh_cargo_lock_ordering, REFRESH_CARGO_LOCK_ORDERING_ENV_VAR,
    };
    use serial_test::serial;
    use std::fs;

    #[test]
    fn test_refresh_cargo_lock_ordering_real_fetch() {
        let temp_dir = tempfile::tempdir().unwrap();
        let rust_dir = temp_dir.path().join("rust");
        fs::create_dir_all(rust_dir.join("src")).unwrap();
        fs::write(
            rust_dir.join("Cargo.toml"),
            "[package]\nname = \"integrator_refresh_test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            rust_dir.join("src/lib.rs"),
            "pub fn answer() -> i32 { 42 }\n",
        )
        .unwrap();

        refresh_cargo_lock_ordering(temp_dir.path(), "rust").unwrap();
    }

    #[test]
    #[serial]
    fn test_should_refresh_cargo_lock_ordering_only_when_env_var_is_one() {
        std::env::remove_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR);
        assert!(!should_refresh_cargo_lock_ordering());

        std::env::set_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR, "true");
        assert!(!should_refresh_cargo_lock_ordering());

        std::env::set_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR, "1");
        assert!(should_refresh_cargo_lock_ordering());

        std::env::remove_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR);
    }

    #[test]
    #[serial]
    fn test_maybe_refresh_cargo_lock_ordering_skips_when_env_var_is_not_one() {
        std::env::remove_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR);

        let temp_dir = tempfile::tempdir().unwrap();
        maybe_refresh_cargo_lock_ordering(temp_dir.path(), "does-not-need-to-exist").unwrap();

        std::env::set_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR, "0");
        maybe_refresh_cargo_lock_ordering(temp_dir.path(), "still-not-used").unwrap();

        std::env::remove_var(REFRESH_CARGO_LOCK_ORDERING_ENV_VAR);
    }
}
