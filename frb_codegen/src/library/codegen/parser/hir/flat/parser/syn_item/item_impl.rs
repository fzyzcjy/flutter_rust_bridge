use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirFlatStruct;
use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::if_then_some;
use crate::utils::namespace::{Namespace, NamespacedName};
use itertools::Itertools;
use syn::{ItemImpl, ItemStruct};

pub(crate) fn parse_syn_item_impl(
    target: &mut HirFlatPack,
    item_impl: ItemImpl,
) -> anyhow::Result<TODO> {
    if item_impl.trait_.is_some() {
        (target.functions).push(parse_trait_impl(item_impl, namespace));
    } else {
        (target.functions).extend(parse_for_struct_or_enum(item_impl, namespace, None));
    }
}

fn parse_for_struct_or_enum(
    item_impl: &ItemImpl,
    namespace: &Namespace,
    trait_def_name: Option<NamespacedName>,
) -> Vec<HirFlatFunction> {
    (item_impl.items.iter())
        .filter_map(|item| if_then_some!(let ImplItem::Fn(ref impl_item_fn) = item, impl_item_fn))
        .map(|impl_item_fn| HirFlatFunction {
            namespace: namespace.clone(),
            owner: HirFlatFunctionOwner::Method {
                item_impl: item_impl.to_owned(),
                trait_def_name: trait_def_name.clone(),
            },
            item_fn: GeneralizedItemFn::ImplItemFn(impl_item_fn.clone()),
        })
        .collect_vec()
}
