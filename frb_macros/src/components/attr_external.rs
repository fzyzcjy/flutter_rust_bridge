use crate::components::encoder::create_frb_encoded_comment;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use syn::{ImplItem, ItemImpl};
use syn::spanned::Spanned;

pub(crate) fn handle_external_impl(attribute: TokenStream, item: TokenStream) -> TokenStream {
    if attribute.to_string() != ATTR_KEYWORD {
        return item;
    }

    let item_string = item.to_string();
    let encoded_original_item =
        create_frb_encoded_comment(&format!("#[frb({})]{}", attribute, &item_string));

    let item_syn: ItemImpl = syn::parse(item.into()).unwrap();
    let original_self_ty = item_syn.self_ty.clone();
    let dummy_struct_ty = compute_dummy_struct_ty(&original_self_ty, &item_string);
    let converted_item = convert_item(item_syn, dummy_struct_ty.clone());

    // eprintln!("attribute={attribute:?} self_ty_string={original_self_ty_string} dummy_struct_name={dummy_struct_name} item={item:#?}");

    quote! {
        #encoded_original_item
        const _: () = ();

        #[cfg(not(frb_expand))]
        #converted_item

        #[cfg(not(frb_expand))]
        pub struct #dummy_struct_ty(pub #original_self_ty);
    }
}

fn compute_dummy_struct_ty(original_self_ty: &syn::Type, item_string: &str) -> syn::Type {
    let original_self_ty_string = quote!(#original_self_ty).to_string();
    let dummy_struct_name = format!(
        "{DUMMY_STRUCT_PREFIX}{}{}",
        hex::encode(original_self_ty_string),
        "TODO",
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

const ATTR_KEYWORD: &str = "external";
const DUMMY_STRUCT_PREFIX: &str = "__external_impl__";
