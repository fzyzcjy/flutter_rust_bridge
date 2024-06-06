use crate::codegen::ir::hir::flat::function::{
    HirFlatFunction, HirFlatFunctionOwner, HirFlatFunctionSource,
};
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::tree::module::HirTreeModuleMeta;
use syn::ItemFn;

pub(crate) fn parse_syn_item_fn(item_fn: ItemFn, meta: &HirTreeModuleMeta) -> HirFlatFunction {
    HirFlatFunction {
        namespace: meta.namespace.clone(),
        owner: HirFlatFunctionOwner::Function,
        item_fn: GeneralizedItemFn::ItemFn(item_fn),
        source: HirFlatFunctionSource::Normal,
    }
}
