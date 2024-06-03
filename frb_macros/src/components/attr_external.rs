use crate::components::encoder::create_frb_encoded_comment;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{ImplItem, ItemImpl};

pub(crate) fn handle_external_impl(attribute: TokenStream, item: TokenStream) -> TokenStream {
    if attribute.to_string() != ATTR_KEYWORD {
        return item;
    }

    let encoded_original_item = create_frb_encoded_comment(&format!(
        "#[frb({})]{}",
        attribute.to_string(),
        &item.to_string()
    ));

    let mut item: ItemImpl = syn::parse(item).unwrap();

    let original_self_ty = item.self_ty.clone();
    let original_self_ty_string = quote!(#original_self_ty).to_string();
    let dummy_struct_name = format!(
        "{DUMMY_STRUCT_PREFIX}{}",
        hex::encode(original_self_ty_string)
    );
    let dummy_struct_ty: syn::Type = syn::parse_str(&dummy_struct_name).unwrap();

    item.self_ty = Box::new(dummy_struct_ty.clone());
    for inner_item in &mut item.items {
        if let ImplItem::Fn(inner_item) = inner_item {
            inner_item.block = syn::parse_str("{ unreachable!() }").unwrap();
        }
    }

    let item = item.to_token_stream();

    // eprintln!("attribute={attribute:?} self_ty_string={original_self_ty_string} dummy_struct_name={dummy_struct_name} item={item:#?}");

    (quote! {
        #encoded_original_item
        const _: () = ();

        #[cfg(not(frb_expand))]
        #item

        #[cfg(not(frb_expand))]
        pub struct #dummy_struct_ty(pub #original_self_ty);
    }).into()
}

const ATTR_KEYWORD: &str = "external";
const DUMMY_STRUCT_PREFIX: &str = "__external_impl__";
