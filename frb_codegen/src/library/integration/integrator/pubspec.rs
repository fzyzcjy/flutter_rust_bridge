use crate::library::commands::flutter::flutter_pub_add;
use crate::misc::{FvmInstallMode, Template};
use anyhow::Result;
use std::path::Path;

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
pub(super) fn pub_add_dependencies(
    enable_integration_test: bool,
    enable_local_dependency: bool,
    rust_crate_name: &str,
    template: &Template,
    fvm_install_mode: FvmInstallMode,
) -> Result<()> {
    // frb-coverage:ignore-end
    match template {
        Template::App => flutter_pub_add(
            &[rust_crate_name, "--path=rust_builder"],
            None,
            fvm_install_mode,
        )?,
        Template::Plugin => flutter_pub_add(
            &["integration_test", "--dev", "--sdk=flutter"],
            Some(Path::new("example")),
            fvm_install_mode,
        )?,
    }

    pub_add_dependency_frb(enable_local_dependency, None, fvm_install_mode)?;

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

pub(crate) fn pub_add_dependency_frb(
    enable_local_dependency: bool,
    pwd: Option<&Path>,
    fvm_install_mode: FvmInstallMode,
) -> Result<()> {
    if enable_local_dependency {
        flutter_pub_add(
            &["flutter_rust_bridge", "--path=../../frb_dart"],
            pwd,
            fvm_install_mode,
        )?;
    } else {
        flutter_pub_add(
            &[concat!("flutter_rust_bridge:", env!("CARGO_PKG_VERSION"))],
            pwd,
            // This argument is plumbing into the shell-command wrapper; command behavior is
            // covered by the integrate workflows.
            // frb-coverage:ignore-start
            fvm_install_mode,
            // frb-coverage:ignore-end
        )?;
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
