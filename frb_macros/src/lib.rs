//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge

use proc_macro::*;
use quote::{quote, ToTokens};
use syn::{ImplItem, ItemImpl};

/// Attribute to guide code generation.
///
/// For what it can do, please refer to <https://github.com/fzyzcjy/flutter_rust_bridge>.
// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
#[proc_macro_attribute]
pub fn frb(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut output = format_frb_attribute(format!("#[frb({attribute})]"));
    let item = strip_frb_attr(handle_external_impl(attribute, item));
    output.extend(item);
    output
}

fn strip_frb_attr(item: TokenStream) -> TokenStream {
    item.into_iter()
        .scan(None, |pound, tok| {
            use TokenTree as TT;
            match (&pound, &tok) {
                (None, TT::Punct(punct)) if punct.as_char() == '#' => {
                    *pound = Some(tok);
                    Some(TokenStream::new())
                }
                (Some(_), TT::Group(group)) if is_frb_bracket(group) => {
                    _ = pound.take();
                    Some(format_frb_attribute(format!("#[{}]", group.stream())))
                }
                (_, TT::Group(group)) => Some(
                    [
                        pound.take(),
                        Some(TT::Group(Group::new(
                            group.delimiter(),
                            strip_frb_attr(group.stream()),
                        ))),
                    ]
                    .into_iter()
                    .flatten()
                    .collect(),
                ),
                _ => Some(tok.into()),
            }
        })
        .collect()
}

fn handle_external_impl(attribute: TokenStream, item: TokenStream) -> TokenStream {
    const ATTR_KEYWORD: &str = "external";
    const DUMMY_STRUCT_PREFIX: &str = "__external_impl__";

    if attribute.to_string() != ATTR_KEYWORD {
        return item;
    }

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

    let mut output: TokenStream = item.to_token_stream().into();
    output.extend(dummy_struct_def);
    output
}

fn is_frb_bracket(group: &Group) -> bool {
    matches!((group.delimiter(), group.stream().into_iter().next()), (Delimiter::Bracket, Some(TokenTree::Ident(ident))) if ident.to_string() == "frb")
}

fn format_frb_attribute(item: String) -> TokenStream {
    format!("#[cfg_attr(frb_expand, doc = r###\"frb_marker: {item}\"###)]",)
        .parse()
        .unwrap()
}
// frb-coverage:ignore-end
