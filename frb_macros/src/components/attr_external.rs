use crate::components::encoder::create_frb_encoded_comment;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{ImplItem, ItemImpl};

pub(crate) fn handle_external_impl(attribute: TokenStream, item: TokenStream) -> TokenStream {
    if attribute.to_string() != ATTR_KEYWORD {
        return item;
    }

    let encoded_original_item = create_frb_encoded_comment(
        "items",
        &format!("#[frb({})]{}", attribute.to_string(), &item.to_string()),
    );

    let mut item: ItemImpl = syn::parse(item).unwrap();

    let original_self_ty = &item.self_ty;
    let original_self_ty_string = quote!(#original_self_ty).to_string();
    let dummy_struct_name = format!(
        "{DUMMY_STRUCT_PREFIX}{}",
        hex::encode(original_self_ty_string)
    );
    let dummy_struct_ty = syn::parse_str(&dummy_struct_name).unwrap();

    let dummy_struct_def: TokenStream = quote! {
        pub struct #dummy_struct_ty(pub #original_self_ty);
    }
    .to_token_stream()
    .into();

    item.self_ty = dummy_struct_ty;
    for inner_item in &mut item.items {
        if let ImplItem::Fn(inner_item) = inner_item {
            inner_item.block = syn::parse_str("{ unreachable!() }").unwrap();
        }
    }

    // eprintln!("attribute={attribute:?} self_ty_string={original_self_ty_string} dummy_struct_name={dummy_struct_name} item={item:#?}");

    let mut output: TokenStream = encoded_original_item;
    output.extend(TokenStream::from(item.to_token_stream()));
    output.extend(dummy_struct_def);
    output
}

const ATTR_KEYWORD: &str = "external";
const DUMMY_STRUCT_PREFIX: &str = "__external_impl__";
