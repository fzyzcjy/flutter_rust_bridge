use proc_macro::TokenStream;
use quote::quote;

// TODO rename
pub(crate) fn format_frb_attribute(item: String) -> TokenStream {
    quote! {
        #[cfg_attr(frb_expand, doc="frb_marker={}")]
    }
}
