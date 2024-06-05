use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStructOrEnum;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::{BaseMerger, CombinedMerger};
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::third_party_override_merger::ThirdPartyOverrideMerger;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::trait_def_default_impl_merger::TraitDefDefaultImplMerger;

pub(crate) mod base;
pub(crate) mod third_party_override_merger;
pub(crate) mod trait_def_default_impl_merger;
pub(crate) mod trait_impl_merger;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    let merger = CombinedMerger(vec![TraitDefDefaultImplMerger, ThirdPartyOverrideMerger]);
    transform_component(&mut pack.functions, |x| merger.merge_functions(x));
    transform_component(&mut pack.structs, |x| merger.merge_struct_or_enums(x));
    transform_component(&mut pack.enums, |x| merger.merge_struct_or_enums(x));
    Ok(pack)
}

fn transform_component<T>(items: &mut Vec<T>, merge: Fn(Vec<T>) -> Vec<T>) {
    TODO
}
