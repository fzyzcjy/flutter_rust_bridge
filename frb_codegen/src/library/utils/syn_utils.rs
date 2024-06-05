pub(crate) fn canonicalize_rust_type(raw: &str) -> anyhow::Result<String> {
    let ast: syn::Type = syn::parse_str(raw)?;
    Ok(quote::quote!(#ast).to_string())
}

pub(crate) fn ty_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string()
}
