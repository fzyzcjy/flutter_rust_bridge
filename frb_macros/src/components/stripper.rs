use crate::components::encoder::create_frb_encoded_comment;
use proc_macro::*;

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
                    Some(create_frb_encoded_comment("attribute", &format!("#[{}]", group.stream())))
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
    matches!((group.delimiter(), group.stream().into_iter().next()), (Delimiter::Bracket, Some(TokenTree::Ident(ident))) if ident.to_string() == "frb")
}
