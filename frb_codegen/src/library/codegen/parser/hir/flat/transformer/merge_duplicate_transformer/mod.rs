use clap::builder::TypedValueParser;
use itertools::Itertools;
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

fn transform_component<T, K>(
    items: Vec<T>,
    key: Fn(&T) -> K,
    merge: impl Fn(Vec<T>) -> Vec<T>,
) -> Vec<T> {
    (items.into_iter())
        .group_by(key)
        .into_iter()
        .map(|(_key, items_of_key)| {
            let items_of_key = items_of_key.collect_vec();
            let merged_items_of_key = merge(items_of_key);
            assert!(!merged_items_of_key.is_empty());

            if merged_items_of_key.len() > 1 {
                log::warn!(
                    "There are still multiple objects with same key after merging, \
                    thus randomly pick one (objects={:?})",
                    merged_items_of_key
                );
            }

            merged_items_of_key[0]
        })
        .collect_vec()
}
