use std::hash::Hash;
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
    transform_component(
        &mut pack.functions,
        |x| x.owner_and_name(),
        |merger, a, b| merger.merge_functions(a, b),
    );
    transform_component(
        &mut pack.structs,
        |x| x.name.clone(),
        |merger, a, b| merger.merge_struct_or_enums(a, b),
    );
    transform_component(
        &mut pack.enums,
        |x| x.name.clone(),
        |merger, a, b| merger.merge_struct_or_enums(a, b),
    );

    Ok(pack)
}

fn transform_component<T, K>(
    items: &mut Vec<T>,
    key: Fn(&T) -> K,
    merge: impl Fn(&dyn BaseMerger, T, T) -> Option<T>,
) {
    *items = transform_component_raw(items, key, merge);
}

fn transform_component_raw<T, K: Eq + Hash>(
    items: Vec<T>,
    key: Fn(&T) -> K,
    merge: impl Fn(&dyn BaseMerger, T, T) -> Option<T>,
) -> Vec<T> {
    let mergers = vec![TraitDefDefaultImplMerger, ThirdPartyOverrideMerger];

    (items.into_iter())
        .group_by(key)
        .into_iter()
        .map(|(_key, items_of_key)| {
            let mut items_of_key = items_of_key.collect_vec();
            for merger in &mergers {
                *items_of_key = merge_vec_by_pair(items_of_key, |a, b| merge(merger, a, b));
            }
            assert!(!items_of_key.is_empty());

            if items_of_key.len() > 1 {
                log::warn!(
                    "There are still multiple objects with same key after merging, \
                    thus randomly pick one (objects={:?})",
                    items_of_key
                );
            }

            merged_items_of_key[0]
        })
        .collect_vec()
}

fn merge_vec_by_pair<T>(mut vec: Vec<T>, merger: impl Fn(T, T) -> Option<T>) -> Vec<T> {
    let act_one_round = || {
        // merge(i,j) may be different from merge(j,i)
        for i in 0..vec.len() {
            for j in 0..vec.len() {
                if i == j {
                    continue;
                }

                if let Some(merged) = merger(vec[i], vec[j]) {
                    // super slow but seems not like a bottleneck so ok
                    *vec = (vec.into_iter().enumerate())
                        .filter(|(item_index, _)| item_index != i && item_index != j)
                        .map(|(_, value)| value)
                        .collect_vec();
                    vec.push(merged);

                    return true;
                }
            }
        }
        false
    };

    while act_one_round() {}
    vec
}
