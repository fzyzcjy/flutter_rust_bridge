//! Main documentation is in https://github.com/fzyzcjy/flutter_rust_bridge

mod components;

use crate::components::attr_external::handle_external_impl;
use crate::components::encoder::create_frb_encoded_comment;
use crate::components::stripper::strip_frb_attr;
use proc_macro::*;

/// Attribute to guide code generation.
///
/// For what it can do, please refer to <https://github.com/fzyzcjy/flutter_rust_bridge>.
// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
#[proc_macro_attribute]
pub fn frb(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let mut output = create_frb_encoded_comment("attribute", &format!("#[frb({attribute})]"));
    output.extend(strip_frb_attr(handle_external_impl(attribute, item)));
    output
}
// frb-coverage:ignore-end
