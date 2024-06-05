use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use crate::if_then_some;
use crate::utils::namespace::{Namespace, NamespacedName};
use itertools::Itertools;
use syn::{ImplItem, ItemFn, ItemImpl};

pub(crate) fn parse_syn_item_fn(item_fn: ItemFn, meta: &HirTreeModuleMeta) -> HirFlatFunction {
    HirFlatFunction {
        namespace: meta.namespace.clone(),
        owner: HirFlatFunctionOwner::Function,
        item_fn: GeneralizedItemFn::ItemFn(item_fn),
    }
}
