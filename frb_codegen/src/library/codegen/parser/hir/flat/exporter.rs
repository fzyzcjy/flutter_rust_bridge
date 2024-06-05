use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use log::debug;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
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
}

fn vec_to_map_with_warn<'a, T, K: Eq + Hash + Display, V: Debug + 'a>(
    items: &'a [T],
    extract_entry: impl Fn(&'a T) -> (K, V),
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
