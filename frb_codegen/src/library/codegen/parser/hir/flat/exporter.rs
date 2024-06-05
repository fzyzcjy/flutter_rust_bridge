use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::HirFlatStruct;
use log::debug;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

impl HirFlatPack {
    pub(crate) fn structs_map(&self) -> HashMap<String, &HirFlatStruct> {
        vec_to_map_with_warn(&self.structs, |x| (x.name.name.clone(), x))
    }

    pub(crate) fn enums_map(&self) -> HashMap<String, &HirFlatStruct> {
        vec_to_map_with_warn(&self.enums, |x| (x.name.name.clone(), x))
    }

    pub(crate) fn types_map(&self) -> HashMap<String, &HirFlatStruct> {
        vec_to_map_with_warn(&self.types, |x| (x.ident.clone(), x.target.clone()))
    }
}

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
