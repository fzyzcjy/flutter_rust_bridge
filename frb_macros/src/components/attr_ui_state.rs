use proc_macro::quote;
use proc_macro2::TokenStream;

// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
pub(crate) fn handle(attribute: TokenStream, item: TokenStream) -> TokenStream {
    quote! {
        #converted_item
    }
}
// frb-coverage:ignore-end
