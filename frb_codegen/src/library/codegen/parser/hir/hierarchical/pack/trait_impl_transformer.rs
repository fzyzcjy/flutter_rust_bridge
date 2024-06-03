use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::hierarchical::traits::HirTrait;
use fern::HashMap;
use crate::codegen::ir::hir::hierarchical::module::HirModule;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    let trait_map = collect_traits(&pack);
    pack.visit_mut(&mut |module| {
        transform_module(module, &trait_map);
    });
    Ok(pack)
}

fn collect_traits(pack: &HirPack) -> HashMap<String, HirTrait> {
    let mut traits = vec![];
    pack.visit(&mut |module| traits.extend(module.content.traits.clone()));
    traits
        .into_iter()
        .map(|t| (t.item_trait.ident.to_string(), t))
        .collect()
}

fn transform_module(module: &mut HirModule, trait_map: &HashMap<String, HirTrait>) {
    TODO
}