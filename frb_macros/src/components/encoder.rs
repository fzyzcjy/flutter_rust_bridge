use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn create_frb_encoded_comment(data: &str) -> TokenStream {
    let encoded = format!(r#"frb_encoded({})"#, hex::encode(data));
    quote! {
        #[cfg_attr(frb_expand, doc=#encoded)]
    }
}
