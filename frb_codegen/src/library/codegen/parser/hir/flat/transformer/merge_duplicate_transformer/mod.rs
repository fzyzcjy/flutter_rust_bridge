use itertools::Itertools;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

use crate::codegen::ir::hir::flat::pack::HirFlatPack;

use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::function_frb_override_merger::FunctionFrbOverrideMerger;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::third_party_override_merger::ThirdPartyOverrideMerger;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::trait_def_default_impl_merger::TraitDefDefaultImplMerger;

pub(crate) mod base;
pub(crate) mod function_frb_override_merger;
pub(crate) mod third_party_override_merger;
pub(crate) mod trait_def_default_impl_merger;

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    transform_component(
        &mut pack.functions,
        |x| x.owner_and_name_for_dedup(),
        |merger, a, b| merger.merge_functions(a, b),
    );
    transform_component(
        &mut pack.structs,
        |x| x.name.name.clone(),
        |merger, a, b| merger.merge_structs(a, b),
    );
    transform_component(
        &mut pack.enums,
        |x| x.name.name.clone(),
        |merger, a, b| merger.merge_enums(a, b),
    );
    transform_component(
        &mut pack.traits,
        |x| x.name.name.clone(),
        |merger, a, b| merger.merge_traits(a, b),
    );

    Ok(pack)
}

fn transform_component<T: Debug + Clone + Serialize, K: Eq + Hash + Debug>(
    items: &mut Vec<T>,
    key: impl Fn(&T) -> K,
    merge: impl Fn(&dyn BaseMerger, &T, &T) -> Option<T>,
) {
    *items = transform_component_raw(items.drain(..).collect_vec(), key, merge);
}

fn transform_component_raw<T: Debug + Clone + Serialize, K: Eq + Hash + Debug>(
    items: Vec<T>,
    key: impl Fn(&T) -> K,
    merge: impl Fn(&dyn BaseMerger, &T, &T) -> Option<T>,
) -> Vec<T> {
    let mergers: Vec<Box<dyn BaseMerger>> = vec![
        Box::new(ThirdPartyOverrideMerger),
        Box::new(FunctionFrbOverrideMerger),
        Box::new(TraitDefDefaultImplMerger),
    ];

    (items.into_iter())
        .into_group_map_by(|x| key(x))
        .into_iter()
        .map(|(key, mut items_of_key)| {
            for merger in &mergers {
                merge_vec_by_pair(&mut items_of_key, |a, b| merge(merger.as_ref(), a, b));
            }
            assert!(!items_of_key.is_empty());

            if items_of_key.len() > 1 {
                log::info!(
                    "There are still multiple objects with same key after merging, \
                    thus randomly pick one. This is an issue only if the object is indeed used. \
                    (key={key:?}, objects={})",
                    (items_of_key.iter())
                        .map(|x| serde_json::to_string(x).unwrap())
                        .join(", "),
                );
            }

            items_of_key[0].to_owned()
        })
        .collect_vec()
}

fn merge_vec_by_pair<T>(vec: &mut Vec<T>, merger: impl Fn(&T, &T) -> Option<T>) {
    let mut act_one_round = || {
        // merge(i,j) may be different from merge(j,i)
        for i in 0..vec.len() {
            for j in 0..vec.len() {
                if i == j {
                    continue;
                }

                if let Some(merged) = merger(&vec[i], &vec[j]) {
                    // super slow but seems not like a bottleneck so ok
                    *vec = (vec.drain(..).enumerate())
                        .filter(|(item_index, _)| *item_index != i && *item_index != j)
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
}
