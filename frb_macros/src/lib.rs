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
    let item_str = item.to_string();
    let mut new_str = String::with_capacity(item_str.len());
    let mut last_end = 0;
    while let Some(start) = item_str[last_end..].find("#[frb(") {
        let start = start + last_end;
        let end = item_str[start..].find(")]").unwrap() + start + 2;
        new_str.push_str(&item_str[last_end..start]);
        new_str.push_str("\n/// frb_marker: ");
        new_str.push_str(&item_str[start..end]);
        new_str.push('\n');
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
// frb-coverage:ignore-end
