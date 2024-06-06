use crate::codegen::ir::hir::flat::pack::{HirFlatPack, HirFlatPackComponentVisitor};

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    pack.visit_components_mut(Visitor);
    Ok(pack)
}

struct Visitor;

impl HirFlatPackComponentVisitor for Visitor {
    fn visit<T>(&self, items: &mut Vec<T>) {
        TODO
    }
}
