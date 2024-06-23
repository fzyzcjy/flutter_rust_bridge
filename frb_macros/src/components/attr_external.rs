use crate::components::encoder::create_frb_encoded_comment;
use md5::{Digest, Md5};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{ImplItem, Item, ItemImpl};

// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
pub(crate) fn handle(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item_string = item.to_string();
    let encoded_original_item =
        create_frb_encoded_comment(&format!("#[frb({})]{}", attribute, &item_string));

    let item_syn: syn::Item = syn::parse(item.into()).unwrap();
    let converted_item = match item_syn {
        Item::Impl(x) => handle_syn_item_impl(x, &item_string),
        x => quote! {
            #[cfg(not(frb_expand))]
            #x
        },
    };

    quote! {
        #encoded_original_item
        const _: () = ();

        #converted_item
    }
}

fn handle_syn_item_impl(item_syn: ItemImpl, item_string: &str) -> TokenStream {
    let original_self_ty = item_syn.self_ty.clone();
    let dummy_struct_ty = compute_dummy_struct_ty(&original_self_ty, item_string);
    let converted_item = convert_item(item_syn, dummy_struct_ty.clone());

    quote! {
        #[cfg(not(frb_expand))]
        #converted_item

        #[cfg(not(frb_expand))]
        pub struct #dummy_struct_ty(pub #original_self_ty);
    }
}

fn compute_dummy_struct_ty(original_self_ty: &syn::Type, item_string: &str) -> syn::Type {
    let original_self_ty_string = quote!(#original_self_ty).to_string();

    let item_string_md5 = Md5::digest(item_string);
    let item_string_md5_value = usize::from_le_bytes(item_string_md5[..8].try_into().unwrap());

    let dummy_struct_name = format!(
        "{DUMMY_STRUCT_PREFIX}{}{}",
        hex::encode(original_self_ty_string),
        item_string_md5_value,
    );
    syn::parse_str(&dummy_struct_name).unwrap()
}

fn convert_item(mut item_syn: syn::ItemImpl, dummy_struct_ty: syn::Type) -> TokenStream {
    item_syn.self_ty = Box::new(dummy_struct_ty);
    for inner_item in &mut item_syn.items {
        if let ImplItem::Fn(inner_item) = inner_item {
            if inner_item.block.stmts.is_empty() {
                inner_item.block = syn::parse_str("{ unreachable!() }").unwrap();
            }
        }
    }
    item_syn.to_token_stream()
}

const DUMMY_STRUCT_PREFIX: &str = "__external_impl__";
// frb-coverage:ignore-end
