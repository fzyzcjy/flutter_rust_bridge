//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge

use proc_macro::*;

/// Attribute to guide code generation.
///
/// For what it can do, have a look at the documentation website.
// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
#[proc_macro_attribute]
pub fn frb(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut output = format_frb_attribute(format!("#[frb({attribute})]"));
    output.extend(item);
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
