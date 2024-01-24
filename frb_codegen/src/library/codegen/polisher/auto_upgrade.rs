use crate::codegen::misc::GeneratorProgressBarPack;
use std::path::Path;

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
    todo!()
}

fn handle_rust(rust_crate_dir: &Path) -> anyhow::Result<()> {
    todo!()
}
