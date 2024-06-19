use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::flat::pack::{HirFlatPack, HirFlatPackComponentVisitor};

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    sort_hir_flat_pack(&mut pack);
    Ok(pack)
}

pub(crate) fn sort_hir_flat_pack(pack: &mut HirFlatPack) {
    pack.visit_components_mut(Visitor);
}

struct Visitor;

impl HirFlatPackComponentVisitor for Visitor {
    fn visit<SK: Ord, T: HirFlatComponent<SK>>(&self, items: &mut Vec<T>) {
        items.sort_by_cached_key(|item| item.sort_key());
    }
}
