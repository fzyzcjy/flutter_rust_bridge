use proc_macro2::TokenStream;
use quote::quote;

// This is surely executed - otherwise how can one use any `#[frb]` macro
// but coverage tool does not think so, possibly because it is done in build time
// frb-coverage:ignore-start
pub(crate) fn create_frb_encoded_comment(data: &str) -> TokenStream {
    let encoded = format!(r#"frb_encoded({})"#, hex::encode(data));
    quote! {
        #[cfg_attr(frb_expand, doc=#encoded)]
    }
}
// frb-coverage:ignore-end
