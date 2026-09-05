use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::integration::integrator::pub_add_dependency_frb;
use crate::library::commands::cargo::cargo_add;
use crate::misc::FvmInstallMode;
use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use anyhow::{anyhow, Result};
use cargo_metadata::VersionReq;
use cargo_toml::{Dependency, Manifest};
use std::path::Path;
use std::str::FromStr;

pub(super) fn execute(
    progress_bar_pack: &GeneratorProgressBarPack,
    config: &PolisherInternalConfig,
) -> Result<()> {
    let _pb = progress_bar_pack.polish_upgrade.start();

    let dart_upgrader = DartUpgrader::new(&config.dart_root)?;
    dart_upgrader.execute(config)?;

    let rust_upgrader = RustUpgrader::new(&config.rust_crate_dir)?;
    rust_upgrader.execute(config)
}

trait Upgrader {
    fn execute(&self, config: &PolisherInternalConfig) -> Result<()> {
        if !self.check()? {
            if config.enable_auto_upgrade {
                self.upgrade(config.fvm_install_mode)?;
            } else {
                log::warn!("Auto upgrader find wrong Dart/Rust flutter_rust_bridge dependency version, please enable `auto_upgrade_dependencies` flag or upgrade manually.");
            }
        }
        Ok(())
    }

    fn check(&self) -> Result<bool>;

    fn upgrade(&self, fvm_install_mode: FvmInstallMode) -> Result<()>;
}

struct DartUpgrader<'a> {
    repository: DartRepository,
    base_dir: &'a Path,
}

impl<'a> DartUpgrader<'a> {
    fn new(base_dir: &'a Path) -> Result<Self> {
        Ok(Self {
            repository: DartRepository::from_path(base_dir)?,
            base_dir,
        })
    }
}

impl Upgrader for DartUpgrader<'_> {
    fn check(&self) -> Result<bool> {
        Ok(self
            .repository
            .has_specified_and_installed(
                "flutter_rust_bridge",
                DartDependencyMode::Main,
                &VersionReq::from_str(concat!("=", env!("CARGO_PKG_VERSION")))?,
            )
            .is_ok())
    }

    fn upgrade(&self, fvm_install_mode: FvmInstallMode) -> Result<()> {
        log::info!("Auto upgrade Dart dependency");
        // This shell-command forwarding path is exercised by integration workflows; llvm-cov
        // does not see the external Dart command behavior as meaningful Rust coverage.
        // frb-coverage:ignore-start
        pub_add_dependency_frb(false, Some(self.base_dir), fvm_install_mode)
        // frb-coverage:ignore-end
    }
}

struct RustUpgrader<'a> {
    dependency: Dependency,
    target_name: Option<String>,
    base_dir: &'a Path,
}

impl<'a> RustUpgrader<'a> {
    fn new(base_dir: &'a Path) -> Result<Self> {
        let manifest = Manifest::from_path(base_dir.join("Cargo.toml"))?;
        let (target_name, dependency) = Self::get_dependency(manifest, "flutter_rust_bridge")?;

        Ok(Self {
            dependency,
            target_name,
            base_dir,
        })
    }

    fn get_dependency(
        manifest: Manifest,
        package_name: &str,
    ) -> Result<(Option<String>, Dependency)> {
        manifest
            .dependencies
            .get(package_name)
            .map(|dep| (None, dep.clone()))
            .into_iter()
            .chain(manifest.target.iter().filter_map(|(name, target)| {
                target
                    .dependencies
                    .get(package_name)
                    .map(|dep| (Some(name.to_owned()), dep.clone()))
            }))
            .next()
            .ok_or_else(|| anyhow!("flutter_rust_bridge not found in Cargo.toml dependencies"))
    }
}

impl Upgrader for RustUpgrader<'_> {
    fn check(&self) -> Result<bool> {
        Ok(self.dependency.req() == concat!("=", env!("CARGO_PKG_VERSION")))
    }

    fn upgrade(&self, _fvm_install_mode: FvmInstallMode) -> Result<()> {
        log::info!("Auto upgrade Rust dependency");

        let mut args = vec![concat!("flutter_rust_bridge@=", env!("CARGO_PKG_VERSION"))];

        let target = self
            .target_name
            .as_ref()
            .map(|name| format!("--target={name}"));

        if let Some(target) = &target {
            args.push(target);
        }

        cargo_add(&args, self.base_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::polisher::internal_config::PolisherInternalConfig;
    use cargo_toml::Manifest;
    use std::cell::Cell;
    use tempfile::tempdir;

    struct RecordingUpgrader {
        check_result: Result<bool>,
        upgrade_result: Result<()>,
        forwarded_mode: Cell<Option<FvmInstallMode>>,
    }

    impl Upgrader for RecordingUpgrader {
        fn check(&self) -> Result<bool> {
            match &self.check_result {
                Ok(value) => Ok(*value),
                Err(error) => Err(anyhow!(error.to_string())),
            }
        }

        fn upgrade(&self, fvm_install_mode: FvmInstallMode) -> Result<()> {
            self.forwarded_mode.set(Some(fvm_install_mode));
            match &self.upgrade_result {
                Ok(()) => Ok(()),
                Err(error) => Err(anyhow!(error.to_string())),
            }
        }
    }

    fn config(enable_auto_upgrade: bool) -> PolisherInternalConfig {
        let temp_dir = tempdir().unwrap();
        PolisherInternalConfig {
            duplicated_c_output_path: vec![],
            dart_format_line_length: 80,
            dart_format: false,
            dart_fix: false,
            rust_format: false,
            add_mod_to_lib: false,
            build_runner: false,
            web_enabled: false,
            dart_output: temp_dir.path().join("dart_output"),
            dart_root: temp_dir.path().join("dart_root"),
            rust_crate_dir: temp_dir.path().join("rust_crate"),
            rust_output_path: temp_dir.path().join("rust_output"),
            c_output_path: None,
            enable_auto_upgrade,
            fvm_install_mode: FvmInstallMode::Skip,
        }
    }

    #[test]
    fn upgrader_forwards_fvm_install_mode_from_config() {
        let config = config(true);
        let upgrader = RecordingUpgrader {
            check_result: Ok(false),
            upgrade_result: Ok(()),
            forwarded_mode: Cell::new(None),
        };

        upgrader.execute(&config).unwrap();

        assert_eq!(upgrader.forwarded_mode.get(), Some(FvmInstallMode::Skip));
    }

    /// Leaves a mismatched dependency untouched when automatic upgrades are disabled.
    #[test]
    fn test_upgrader_skips_upgrade_when_disabled() {
        let upgrader = RecordingUpgrader {
            check_result: Ok(false),
            upgrade_result: Ok(()),
            forwarded_mode: Cell::new(None),
        };

        upgrader.execute(&config(false)).unwrap();

        assert_eq!(upgrader.forwarded_mode.get(), None);
    }

    /// Does not upgrade a dependency that already has the current version.
    #[test]
    fn test_upgrader_skips_current_dependency() {
        let upgrader = RecordingUpgrader {
            check_result: Ok(true),
            upgrade_result: Ok(()),
            forwarded_mode: Cell::new(None),
        };

        upgrader.execute(&config(true)).unwrap();

        assert_eq!(upgrader.forwarded_mode.get(), None);
    }

    /// Propagates failures from dependency checking and upgrade commands.
    #[test]
    fn test_upgrader_propagates_check_and_upgrade_errors() {
        let check_error = RecordingUpgrader {
            check_result: Err(anyhow!("check failed")),
            upgrade_result: Ok(()),
            forwarded_mode: Cell::new(None),
        };
        let upgrade_error = RecordingUpgrader {
            check_result: Ok(false),
            upgrade_result: Err(anyhow!("upgrade failed")),
            forwarded_mode: Cell::new(None),
        };

        assert!(check_error.execute(&config(true)).is_err());
        assert!(upgrade_error.execute(&config(true)).is_err());
    }

    fn parse_manifest(text: &str) -> Manifest {
        toml::from_str(text).unwrap()
    }

    #[test]
    fn test_get_dependency_from_target_specific_dependencies() {
        let manifest = parse_manifest(
            r#"
                [package]
                name = "demo"
                version = "0.1.0"
                edition = "2021"

                [target.x86_64-unknown-linux-gnu.dependencies]
                flutter_rust_bridge = "=1.0.0"
            "#,
        );

        let (target_name, dependency) =
            RustUpgrader::get_dependency(manifest, "flutter_rust_bridge").unwrap();

        assert_eq!(target_name.as_deref(), Some("x86_64-unknown-linux-gnu"));
        assert_eq!(dependency.req(), "=1.0.0");
    }

    #[test]
    fn test_get_dependency_prefers_root_dependencies() {
        let manifest = parse_manifest(
            r#"
                [package]
                name = "demo"
                version = "0.1.0"
                edition = "2021"

                [dependencies]
                flutter_rust_bridge = "=2.0.0"

                [target.x86_64-unknown-linux-gnu.dependencies]
                flutter_rust_bridge = "=1.0.0"
            "#,
        );

        let (target_name, dependency) =
            RustUpgrader::get_dependency(manifest, "flutter_rust_bridge").unwrap();

        assert_eq!(target_name, None);
        assert_eq!(dependency.req(), "=2.0.0");
    }

    /// Reports a manifest that does not declare flutter_rust_bridge.
    #[test]
    fn test_get_dependency_reports_missing_dependency() {
        let manifest = parse_manifest(
            r#"
                [package]
                name = "demo"
                version = "0.1.0"
                edition = "2021"
            "#,
        );

        assert!(RustUpgrader::get_dependency(manifest, "flutter_rust_bridge").is_err());
    }

    /// Distinguishes current and old Rust dependency requirements.
    #[test]
    fn test_rust_upgrader_checks_current_and_old_versions() {
        let temp_dir = tempdir().unwrap();
        let current_dir = temp_dir.path().join("current");
        let old_dir = temp_dir.path().join("old");
        std::fs::create_dir_all(&current_dir).unwrap();
        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::write(
            current_dir.join("Cargo.toml"),
            format!(
                "[package]\nname = \"current\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nflutter_rust_bridge = \"={}\"\n",
                env!("CARGO_PKG_VERSION")
            ),
        )
        .unwrap();
        std::fs::write(
            old_dir.join("Cargo.toml"),
            "[package]\nname = \"old\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nflutter_rust_bridge = \"=0.0.1\"\n",
        )
        .unwrap();

        assert!(RustUpgrader::new(&current_dir).unwrap().check().unwrap());
        assert!(!RustUpgrader::new(&old_dir).unwrap().check().unwrap());
    }
}
