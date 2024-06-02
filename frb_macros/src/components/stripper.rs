use crate::components::encoder::format_frb_attribute;
use proc_macro::*;

pub(crate) fn strip_frb_attr(item: TokenStream) -> TokenStream {
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

fn is_frb_bracket(group: &Group) -> bool {
    matches!((group.delimiter(), group.stream().into_iter().next()), (Delimiter::Bracket, Some(TokenTree::Ident(ident))) if ident.to_string() == "frb")
}
