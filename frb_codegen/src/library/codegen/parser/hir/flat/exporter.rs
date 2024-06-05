use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

impl HirFlatPack {}

fn vec_to_map_with_warn<T, K: Eq + Hash, V>(
    items: &[T],
    extract_entry: impl Fn(&T) -> (K, V),
) -> HashMap<K, V> {
    let mut ans = HashMap::new();
    for item in items {
        let (key, value) = extract_entry(item);
        if let Some(old_value) = ans.get(&key) {
            debug!("Same key={key} has multiple values: {old_value:?} (thrown away) and {value:?} (used). This may or may not be a problem.");
        }
        ans.insert(key, value);
    }
    ans
}
