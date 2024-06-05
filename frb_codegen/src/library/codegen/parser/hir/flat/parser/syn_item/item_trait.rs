use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::flat::traits::HirFlatTrait;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::utils::namespace::{Namespace, NamespacedName};
use syn::ItemImpl;
use syn::ItemTrait;

pub(crate) fn parse_syn_item_trait(
    target: &mut HirFlatPack,
    item_trait: ItemTrait,
    meta: &HirTreeModuleMeta,
) -> HirTrait {
    target.traits.push(parse_trait(&item_trait, meta));
    target.functions.extend(TODO);
}

fn parse_trait(item_trait: &ItemTrait, meta: &HirTreeModuleMeta) -> HirFlatTrait {
    HirFlatTrait {
        name: NamespacedName::new(namespace.clone(), item_trait.ident.to_string()),
    }
}
