use crate::codegen::ir::hir::misc::item_fn::GeneralizedItemFn;
use crate::utils::syn_utils::ty_to_string;
use itertools::Itertools;
use quote::ToTokens;
use serde::{Serialize, Serializer};

pub(crate) fn serialize_syn<T: ToTokens, S: Serializer>(
    value: &T,
    s: S,
) -> Result<S::Ok, S::Error> {
    quote::quote!(#value).to_string().serialize(s)
}

pub(crate) fn serialize_vec_syn<T: ToTokens, S: Serializer>(
    values: &[T],
    s: S,
) -> Result<S::Ok, S::Error> {
    let values = values
        .iter()
        .map(|value| quote::quote!(#value).to_string())
        .collect_vec();
    values.serialize(s)
}

// pub(crate) fn serialize_item_trait<S: Serializer>(x: &ItemTrait, s: S) -> Result<S::Ok, S::Error> {
//     s.serialize_str(&format!("ItemTrait(ident={})", x.ident))
// }
//
// pub(crate) fn serialize_item_impl<S: Serializer>(x: &ItemImpl, s: S) -> Result<S::Ok, S::Error> {
//     s.serialize_str(&format!(
//         "ItemImpl(self_ty={}, trait={})",
//         ty_to_string(&x.self_ty),
//         x.trait_
//             .as_ref()
//             .map(|t| ty_to_string(&t.1).replace(' ', ""))
//             .unwrap_or("None".to_owned())
//     ))
// }

pub(crate) fn serialize_generalized_item_fn<S: Serializer>(
    x: &GeneralizedItemFn,
    s: S,
) -> Result<S::Ok, S::Error> {
    s.serialize_str(&format!(
        "GeneralizedItemFn(name={}, vis={:?}, attrs=[{}])",
        x.name(),
        x.vis_raw(),
        x.attrs().iter().map(ty_to_string).join(", "),
    ))
}
