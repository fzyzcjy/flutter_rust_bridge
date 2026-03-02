use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::struct_or_enum::{HirFlatEnum, HirFlatStruct};
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
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
        // Filter out traits marked with #[frb(ignore)]
        let non_ignored: Vec<_> = self
            .traits
            .iter()
            .filter(|t| {
                !FrbAttributes::parse(&t.attrs)
                    .map(|a| a.ignore())
                    .unwrap_or(false)
            })
            .collect();
        vec_to_map_with_warn(&non_ignored, |x| (x.name.name.clone(), *x))
    }

    /// Returns the names of traits that are marked with #[frb(ignore)].
    /// Used to properly handle trait object types for ignored traits.
    pub(crate) fn ignored_trait_names(&self) -> std::collections::HashSet<String> {
        let ignored: std::collections::HashSet<_> = self.traits
            .iter()
            .filter(|t| {
                FrbAttributes::parse(&t.attrs)
                    .map(|a| a.ignore())
                    .unwrap_or(false)
            })
            .map(|t| t.name.name.clone())
            .collect();
        if !ignored.is_empty() {
            log::debug!("Ignored trait names: {:?}", ignored);
        }
        ignored
    }

    pub(crate) fn types_map(&self) -> HashMap<String, Type> {
        // Only include non-generic type aliases here.
        // Generic type aliases are handled separately via generic_types_map()
        // to support proper generic parameter substitution.
        vec_to_map_with_warn(
            &self.types.iter().filter(|x| x.generics.is_none()).collect::<Vec<_>>(),
            |x| (x.ident.clone(), x.target.clone()),
        )
    }

    pub(crate) fn generic_types_map(&self) -> HashMap<String, &HirFlatTypeAlias> {
        vec_to_map_with_warn(
            &self.types.iter().filter(|x| x.generics.is_some()).collect::<Vec<_>>(),
            |x| (x.ident.clone(), *x),
        )
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
