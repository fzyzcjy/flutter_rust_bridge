use crate::codegen::ir::hir::hierarchical::pack::HirPack;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    pack.visit_mut(&mut |module| {
        todo!();
    });
    Ok(pack)
}
