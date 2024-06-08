use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use syn::ItemFn;

pub(crate) fn parse_syn_item_fn(item_fn: ItemFn, meta: &HirNaiveFlatItemMeta) -> HirFlatFunction {
    HirFlatFunction {
        namespace: meta.namespace.clone(),
        owner: HirFlatFunctionOwner::Function,
        item_fn: GeneralizedItemFn::ItemFn(item_fn),
        sources: meta.sources.clone(),
    }
}
