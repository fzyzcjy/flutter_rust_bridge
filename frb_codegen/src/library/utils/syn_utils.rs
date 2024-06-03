pub(crate) fn canonicalize_rust_type(raw: &str) -> anyhow::Result<String> {
    let ast: syn::Type = syn::parse_str(raw)?;
    Ok(quote::quote!(#ast).to_string())
}
