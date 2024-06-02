use proc_macro::TokenStream;

// TODO rename
pub(crate) fn format_frb_attribute(item: String) -> TokenStream {
    format!("#[cfg_attr(frb_expand, doc = r###\"frb_marker: {item}\"###)]",)
        .parse()
        .unwrap()
}
