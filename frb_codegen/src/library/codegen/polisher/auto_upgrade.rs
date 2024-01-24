use crate::codegen::misc::GeneratorProgressBarPack;
use crate::integration::integrator::pub_add_dependency_frb;
use crate::library::commands::cargo::cargo_add;
use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use crate::utils::path_utils::path_to_string;
use anyhow::Result;
use cargo_metadata::VersionReq;
use std::path::Path;
use std::str::FromStr;

pub(super) fn execute(
    progress_bar_pack: &GeneratorProgressBarPack,
    dart_root: &Path,
    rust_crate_dir: &Path,
) -> Result<()> {
    let _pb = progress_bar_pack.polish_upgrade.start();
    DartUpgrader::execute(dart_root)?;
    RustUpgrader::execute(rust_crate_dir)
}

trait Upgrader {
    fn execute(base_dir: &Path) -> Result<()> {
        if !Self::check(base_dir)? {
            Self::upgrade(base_dir)?;
        }
        Ok(())
    }

    fn check(base_dir: &Path) -> Result<bool>;

    fn upgrade(base_dir: &Path) -> Result<()>;
}

struct DartUpgrader;

impl Upgrader for DartUpgrader {
    fn check(base_dir: &Path) -> Result<bool> {
        let repo = DartRepository::from_str(&path_to_string(base_dir)?)?;
        Ok(repo
            .has_specified_and_installed(
                "flutter_rust_bridge",
                DartDependencyMode::Main,
                &VersionReq::from_str(&format!("={}", env!("CARGO_PKG_VERSION")))?,
            )
            .is_ok())
    }

    fn upgrade(base_dir: &Path) -> Result<()> {
        pub_add_dependency_frb(false, Some(base_dir))
    }
}

struct RustUpgrader;

impl Upgrader for RustUpgrader {
    fn check(base_dir: &Path) -> Result<bool> {
        let manifest = cargo_toml::Manifest::from_path(base_dir.join("Cargo.toml"))?;
        let dep = &manifest.dependencies["flutter_rust_bridge"];
        Ok(dep.req() == format!("={}", env!("CARGO_PKG_VERSION")))
    }

    fn upgrade(base_dir: &Path) -> Result<()> {
        cargo_add(
            &[format!(
                "flutter_rust_bridge@={}",
                env!("CARGO_PKG_VERSION")
            )],
            base_dir,
        )
    }
}
