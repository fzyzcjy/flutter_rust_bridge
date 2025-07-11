use crate::codegen::misc::GeneratorProgressBarPack;
use crate::integration::integrator::pub_add_dependency_frb;
use crate::library::commands::cargo::cargo_add;
use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use anyhow::{anyhow, Result};
use cargo_metadata::VersionReq;
use cargo_toml::{Dependency, Manifest};
use std::path::Path;
use std::str::FromStr;

pub(super) fn execute(
    progress_bar_pack: &GeneratorProgressBarPack,
    dart_root: &Path,
    rust_crate_dir: &Path,
    enable_auto_upgrade: bool,
) -> Result<()> {
    let _pb = progress_bar_pack.polish_upgrade.start();

    let dart_upgrader = DartUpgrader::new(dart_root)?;
    dart_upgrader.execute(enable_auto_upgrade)?;

    let rust_upgrader = RustUpgrader::new(rust_crate_dir)?;
    rust_upgrader.execute(enable_auto_upgrade)
}

trait Upgrader {
    fn execute(&self, enable_auto_upgrade: bool) -> Result<()> {
        if !self.check()? {
            if enable_auto_upgrade {
                self.upgrade()?;
            } else {
                log::warn!("Auto upgrader find wrong Dart/Rust flutter_rust_bridge dependency version, please enable `auto_upgrade_dependencies` flag or upgrade manually.");
            }
        }
        Ok(())
    }

    fn check(&self) -> Result<bool>;

    fn upgrade(&self) -> Result<()>;
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

    fn upgrade(&self) -> Result<()> {
        log::info!("Auto upgrade Dart dependency");
        pub_add_dependency_frb(false, Some(self.base_dir))
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

    fn upgrade(&self) -> Result<()> {
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
