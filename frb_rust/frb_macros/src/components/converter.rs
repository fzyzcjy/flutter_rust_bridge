use crate::components::encoder::create_frb_encoded_comment;
use proc_macro2::*;

// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
pub(crate) fn convert_frb_attr_to_encoded_form(item: TokenStream) -> TokenStream {
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
                    Some(create_frb_encoded_comment(&format!(
                        "#[{}]",
                        group.stream()
                    )))
                }
                (_, TT::Group(group)) => Some(
                    [
                        pound.take(),
                        Some(TT::Group(Group::new(
                            group.delimiter(),
                            convert_frb_attr_to_encoded_form(group.stream()),
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

fn is_frb_bracket(group: &Group) -> bool {
    matches!((group.delimiter(), group.stream().into_iter().next()), (Delimiter::Bracket, Some(TokenTree::Ident(ident))) if ident == "frb")
}
// frb-coverage:ignore-end
