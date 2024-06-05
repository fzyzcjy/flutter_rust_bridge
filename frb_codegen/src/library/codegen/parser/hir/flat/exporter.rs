use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;

impl HirFlatPack {}

fn vec_to_map_with_warn<T, K, V>(items: &[T], key_value: impl Fn(&T) -> (K, V)) -> HashMap<K, V> {
    items.iter().map(|item| key_value(item))
        .collect()
}
