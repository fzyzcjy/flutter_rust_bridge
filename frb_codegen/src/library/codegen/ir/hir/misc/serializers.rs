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

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks Syn value/vector encoding and function metadata fields.
    #[test]
    fn serializes_syn_values_and_function_metadata() {
        let item: syn::ItemStruct = syn::parse_str("pub struct Example<T>(T);").unwrap();
        assert_eq!(
            serde_json::to_string(&SynValue(&item)).unwrap(),
            "\"pub struct Example < T > (T) ;\""
        );

        let values = vec![
            syn::parse_str::<syn::Type>("u8").unwrap(),
            syn::parse_str::<syn::Type>("String").unwrap(),
        ];
        assert_eq!(
            serde_json::to_string(&SynValues(&values)).unwrap(),
            "[\"u8\",\"String\"]"
        );

        let function = GeneralizedItemFn::ItemFn(
            syn::parse_str("#[inline] pub fn example(value: u8) {}").unwrap(),
        );
        let serialized = serde_json::to_string(&SerializableFunction(&function)).unwrap();
        assert!(serialized.contains("GeneralizedItemFn(name=example"));
        assert!(serialized.contains("attrs=["));
        assert!(serialized.contains("inline"));
    }

    struct SynValue<'a>(&'a syn::ItemStruct);

    impl serde::Serialize for SynValue<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serialize_syn(self.0, serializer)
        }
    }

    struct SynValues<'a>(&'a [syn::Type]);

    impl serde::Serialize for SynValues<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serialize_vec_syn(self.0, serializer)
        }
    }

    struct SerializableFunction<'a>(&'a GeneralizedItemFn);

    impl serde::Serialize for SerializableFunction<'_> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serialize_generalized_item_fn(self.0, serializer)
        }
    }
}
