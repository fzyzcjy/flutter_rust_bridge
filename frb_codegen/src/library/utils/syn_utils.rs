use quote::ToTokens;

pub(crate) fn canonicalize_rust_type(raw: &str) -> anyhow::Result<String> {
    let ast: syn::Type = syn::parse_str(raw)?;
    Ok(quote::quote!(#ast).to_string())
}

// TODO rename
pub(crate) fn ty_to_string<T: ToTokens>(ty: &T) -> String {
    quote::quote!(#ty).to_string()
}
