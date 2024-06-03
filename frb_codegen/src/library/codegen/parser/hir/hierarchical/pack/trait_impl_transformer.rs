use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    for hir_crate in pack.crates.iter_mut() {
        transform_module(&mut hir_crate.root_module)?;
    }
    Ok(pack)
}

fn transform_module(target: &mut HirModule) -> anyhow::Result<()> {
    TODO;
}
