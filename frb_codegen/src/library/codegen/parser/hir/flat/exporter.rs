use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::utils::namespace::NamespacedName;
use log::debug;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use syn::Type;

impl HirFlatPack {
    pub(crate) fn structs_map(&self) -> HashMap<String, &HirFlatStruct> {
        vec_to_map_with_warn(&self.structs, |x| (x.name.name.clone(), x))
    }

    pub(crate) fn enums_map(&self) -> HashMap<String, &HirFlatEnum> {
        vec_to_map_with_warn(&self.enums, |x| (x.name.name.clone(), x))
    }

    pub(crate) fn traits_map(&self) -> HashMap<String, &HirFlatTrait> {
        vec_to_map_with_warn(&self.traits, |x| (x.name.name.clone(), x))
    }

    pub(crate) fn types_map(&self) -> HashMap<String, Type> {
        vec_to_map_with_warn(&self.types, |x| (x.ident.clone(), x.target.clone()))
    }

    pub(crate) fn structs_namespaced_map(&self) -> HashMap<NamespacedName, &HirFlatStruct> {
        vec_to_map_with_warn(&self.structs, |x| (x.name.clone(), x))
    }

    pub(crate) fn enums_namespaced_map(&self) -> HashMap<NamespacedName, &HirFlatEnum> {
        vec_to_map_with_warn(&self.enums, |x| (x.name.clone(), x))
    }
}

fn vec_to_map_with_warn<'a, T, K: Eq + Hash + Debug, V: Debug + 'a>(
    items: &'a [T],
    extract_entry: impl Fn(&'a T) -> (K, V),
) -> HashMap<K, V> {
    let mut ans = HashMap::new();
    for item in items {
        let (key, value) = extract_entry(item);
        match ans.entry(key) {
            Entry::Occupied(mut entry) => {
                let old_value = entry.insert(value);
                debug!("Same key={:?} has multiple values: {old_value:?} (thrown away) and {:?} (used). This may or may not be a problem.", entry.key(), entry.get());
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
    }
    ans
}
