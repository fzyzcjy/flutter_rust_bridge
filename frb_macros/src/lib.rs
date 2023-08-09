//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge

use proc_macro::*;

fn remove_marker_attr(input: TokenStream, ident: &str) -> TokenStream {
    use TokenTree as tt;
    input
        .into_iter()
        .scan(None, |state, x| match (state, x) {
            (pound @ None, tt::Punct(p)) if p.as_char() == '#' => {
                *pound = Some(tt::Punct(p));
                Some(vec![])
            }
            (p @ Some(_), tt::Group(g)) => match g.delimiter() {
                Delimiter::Bracket => match g.stream().into_iter().next() {
                    Some(tt::Ident(i)) if i.to_string() == ident => {
                        let _ = p.take();
                        Some(vec![])
                    }
                    Some(_) => Some(vec![p.take().unwrap(), tt::Group(g)]),
                    _ => Some(vec![tt::Group(g)]),
                },
                _ => Some(vec![p.take().unwrap(), tt::Group(g)]),
            },
            (None, tt::Group(g)) => Some(vec![tt::Group(Group::new(
                g.delimiter(),
                remove_marker_attr(g.stream(), ident),
            ))]),
            (_, x) => Some(vec![x]),
        })
        .flatten()
        .collect()
}

/// Attribute to guide code generation.
///
/// For what it can do, have a look at the documentation website.
#[proc_macro_attribute]
pub fn frb(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item_str = item.to_string();
    let mut new_str = String::with_capacity(item_str.len());
    let mut last_end = 0;
    while let Some(start) = item_str[last_end..].find("#[frb(") {
        let start = start + last_end;
        let end = item_str[start..].find(")]").unwrap() + start + 2;
        new_str.push_str(&item_str[last_end..start]);
        new_str.push_str("\n/// frb_marker: ");
        new_str.push_str(&item_str[start..end]);
        new_str.push_str("\n");
        last_end = end;
    }
    new_str.push_str(&item_str[last_end..]);
    let item: TokenStream = new_str.parse().unwrap();

    let attr = attribute.to_string().replace('\n', "");
    let comment_str = format!("/// frb_marker: #[frb({attr})]");
    let mut comment: TokenStream = comment_str.parse().unwrap();
    comment.extend([item]);
    comment
}
