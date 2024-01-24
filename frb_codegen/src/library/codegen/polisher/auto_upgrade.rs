use crate::codegen::misc::GeneratorProgressBarPack;

pub(super) fn execute(progress_bar_pack: &GeneratorProgressBarPack) -> anyhow::Result<()> {
    let _pb = progress_bar_pack.polish_upgrade.start();
    todo!()
}
