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
    if &attribute.to_string() != ATTR_KEYWORD {
        return item;
    }

    eprintln!("attribute={attribute:?} item={item:?}");
    todo!()
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
