use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use itertools::Itertools;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    transform_component(&mut pack.functions);
    transform_component(&mut pack.structs);
    transform_component(&mut pack.enums);
    Ok(pack)
}

fn transform_component<T>(items: &mut Vec<T>) {
    *items = (items.drain(..))
        .map(|item| {
            TODO;
            TODO;
        })
        .collect_vec();
}
