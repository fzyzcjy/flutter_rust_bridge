use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::hir::flat::transformer::merge_duplicate_transformer::base::BaseMerger;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use itertools::Itertools;
use serde::Serialize;
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) fn transform(
    mut pack: HirFlatPack,
    config: &ParserHirInternalConfig,
) -> anyhow::Result<HirFlatPack> {
    transform_component(&mut pack.functions, |x| x.is_public().unwrap());
    Ok(pack)
}

fn transform_component<T>(items: &mut Vec<T>, filter: impl Fn(&T) -> bool) {
    *items = items.drain(..).filter(|x| filter(x)).collect_vec();
}
