use crate::codegen::ir::hir::flat::constant::HirFlatConstant;
use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use syn::{ItemConst, ItemFn};

pub(crate) fn parse_syn_item_const(
    item_const: ItemConst,
    meta: &HirNaiveFlatItemMeta,
) -> HirFlatConstant {
    HirFlatConstant {
        namespace: meta.namespace.clone(),
        item_const,
    }
}
