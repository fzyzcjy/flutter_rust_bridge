use crate::codegen::misc::GeneratorProgressBarPack;
use crate::utils::dart_repository::dart_repo::{DartDependencyMode, DartRepository};
use crate::utils::path_utils::path_to_string;
use semver::VersionReq;
use std::path::Path;
use std::str::FromStr;

pub(super) fn execute(
    progress_bar_pack: &GeneratorProgressBarPack,
    dart_root: &Path,
    rust_crate_dir: &Path,
) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_upgrade.start();
    handle_dart(dart_root)?;
    handle_rust(rust_crate_dir)
}

fn handle_dart(dart_root: &Path) -> anyhow::Result<()> {
    let repo = DartRepository::from_str(&path_to_string(dart_root)?)?;
    let pass = repo
        .has_specified_and_installed(
            "flutter_rust_bridge",
            DartDependencyMode::Main,
            &VersionReq::from_str(&format!("={}", env!("CARGO_PKG_VERSION")))?,
        )
        .is_ok();
    if !pass {
        todo!()
    }
    Ok(())
}

fn handle_rust(rust_crate_dir: &Path) -> anyhow::Result<()> {
    todo!()
}
