use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::trait_impl::HirFlatTraitImpl;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use itertools::{concat, Itertools};

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    for trait_impl in &pack.trait_impls {
        pack.functions.extend(compute_functions(trait_impl, &pack));
    }
    Ok(pack)
}

fn compute_functions(trait_impl: &HirFlatTraitImpl, pack: &HirFlatPack) -> Vec<HirFlatFunction> {
    (pack.functions.iter())
        .filter(|f| {
            if let HirFlatFunctionOwner::TraitDef { trait_def_name } = &f.owner {
                trait_def_name.name == trait_impl.trait_name
            } else {
                false
            }
        })
        .map(|f| HirFlatFunction {
            namespace: f.namespace.clone(), // TODO correct?
            owner: HirFlatFunctionOwner::StructOrEnum {
                impl_ty: trait_impl.impl_ty.clone(),
                trait_def_name: Some(trait_impl.trait_name.clone()),
            },
            sources: concat([
                f.sources.clone(),
                vec![HirGenerationSource::CopyFromTraitDef],
            ]),
            item_fn: f.item_fn.clone(),
        })
        .collect_vec()
}
