use crate::library::commands::flutter::flutter_pub_add;
use crate::misc::{FvmInstallMode, IntegrationBackend, Template};
use anyhow::Result;
use std::path::Path;

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
pub(super) fn pub_add_dependencies(
    enable_integration_test: bool,
    enable_local_dependency: bool,
    rust_crate_name: &str,
    template: &Template,
    integration_backend: IntegrationBackend,
    fvm_install_mode: FvmInstallMode,
) -> Result<()> {
    // frb-coverage:ignore-end
    if integration_backend == IntegrationBackend::Cargokit {
        let (dependency_working_directory, hooks_local_path) = match template {
            Template::App => (Some(Path::new("rust_builder")), "../../../frb_hooks"),
            Template::Plugin => (None, "../../frb_hooks"),
        };
        flutter_pub_add(
            &["code_assets:^1.0.0"],
            dependency_working_directory,
            fvm_install_mode,
        )?;
        pub_add_dependency(
            "flutter_rust_bridge_hooks",
            hooks_local_path,
            enable_local_dependency,
            dependency_working_directory,
            fvm_install_mode,
        )?;
        if matches!(template, Template::App) {
            flutter_pub_add(
                &[rust_crate_name, "--path=rust_builder"],
                None,
                fvm_install_mode,
            )?;
        }
    }

    pub_add_dependency_frb(enable_local_dependency, None, fvm_install_mode)?;
    if integration_backend == IntegrationBackend::NativeAssets {
        pub_add_dependency_frb_hooks(enable_local_dependency, None, fvm_install_mode)?;
    }

    if matches!(template, Template::Plugin) {
        flutter_pub_add(
            &["integration_test", "--dev", "--sdk=flutter"],
            Some(Path::new("example")),
            fvm_install_mode,
        )?;
    }

    // // Temporarily avoid `^` before https://github.com/flutter/flutter/issues/84270 is fixed
    // flutter_pub_add(&["ffigen:8.0.2", "--dev"])?;

    if enable_integration_test {
        flutter_pub_add(
            &["integration_test", "--dev", "--sdk=flutter"],
            None,
            fvm_install_mode,
        )?;
        // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
        // frb-coverage:ignore-start
    }
    // frb-coverage:ignore-end

    Ok(())
}

pub(crate) fn pub_add_dependency_frb_hooks(
    enable_local_dependency: bool,
    pwd: Option<&Path>,
    fvm_install_mode: FvmInstallMode,
) -> Result<()> {
    pub_add_dependency(
        "flutter_rust_bridge_hooks",
        "../../frb_hooks",
        enable_local_dependency,
        pwd,
        fvm_install_mode,
    )
}

pub(crate) fn pub_add_dependency_frb(
    enable_local_dependency: bool,
    pwd: Option<&Path>,
    fvm_install_mode: FvmInstallMode,
) -> Result<()> {
    pub_add_dependency(
        "flutter_rust_bridge",
        "../../frb_dart",
        enable_local_dependency,
        pwd,
        fvm_install_mode,
    )
}

fn pub_add_dependency(
    package: &'static str,
    local_path: &'static str,
    enable_local_dependency: bool,
    pwd: Option<&Path>,
    fvm_install_mode: FvmInstallMode,
) -> Result<()> {
    if enable_local_dependency {
        let path_arg = format!("--path={local_path}");
        flutter_pub_add(&[package, path_arg.as_str()], pwd, fvm_install_mode)?;
    } else {
        // This release dependency path is plumbing into the shell-command wrapper; command behavior
        // is covered by the integrate workflows.
        // frb-coverage:ignore-start
        let version_arg = format!("{package}:{}", env!("CARGO_PKG_VERSION"));
        flutter_pub_add(&[version_arg.as_str()], pwd, fvm_install_mode)?;
        // frb-coverage:ignore-end
    };

    Ok(())
}

pub(super) fn add_publish_to_none(dart_root: &Path) -> Result<()> {
    let path = dart_root.join("pubspec.yaml");
    let text_raw = std::fs::read_to_string(&path)?;
    let text_output = format!("publish_to: none\n{text_raw}");
    std::fs::write(&path, text_output)?;
    Ok(())
}
