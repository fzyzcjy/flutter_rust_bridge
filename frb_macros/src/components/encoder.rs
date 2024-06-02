use proc_macro::TokenStream;

// TODO rename
pub(crate) fn format_frb_attribute(item: String) -> TokenStream {
    format!(r###"#[cfg_attr(frb_expand, doc="frb_marker={}"]"###)
        .parse()
        .unwrap()
}
