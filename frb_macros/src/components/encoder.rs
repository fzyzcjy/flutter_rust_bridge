use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn create_frb_encoded_comment(mode: &str, data: &str) -> TokenStream {
    let encoded = format!(r#"frb_encoded({},{})"#, mode, hex::encode(data));
    quote! {
        #[cfg_attr(frb_expand, doc=#encoded)]
    }.into()
}
