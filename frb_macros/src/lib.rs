//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge

mod components;

use crate::components::attr_external::handle_attr_external;
use crate::components::converter::convert_frb_attr_to_encoded_form;
use crate::components::encoder::create_frb_encoded_comment;
use proc_macro::TokenStream;

/// Attribute to guide code generation.
///
/// For what it can do, please refer to <https://github.com/fzyzcjy/flutter_rust_bridge>.
// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
#[proc_macro_attribute]
pub fn frb(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute_encoded = create_frb_encoded_comment(&format!("#[frb({attribute})]"));

    let item_converted = item.into();
    let item_converted = handle_attr_external(attribute.into(), item_converted);
    let item_converted = convert_frb_attr_to_encoded_form(item_converted);

    (quote::quote! {
        #attribute_encoded
        #item_converted
    })
    .into()
}
// frb-coverage:ignore-end
