use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::hierarchical::traits::HirTrait;
use fern::HashMap;
use itertools::Itertools;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    let trait_map = collect_traits(&pack);
    pack.visit_mut(&mut |module| {
        (module.content.functions).extend(compute_methods(module, &trait_map));
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

fn compute_methods(module: &HirModule, trait_map: &HashMap<String, HirTrait>) -> Vec<HirFunction> {
    (module.content.trait_impls.iter())
        .flat_map(|trait_impl| {
            let trait_name_raw = trait_impl.item_impl.trait_.unwrap().1;
            let trait_name = trait_name_raw.segments.last().unwrap().ident.to_string();
            let trait_def = trait_map.get(trait_name);
            TODO
        })
        .collect_vec()
}
