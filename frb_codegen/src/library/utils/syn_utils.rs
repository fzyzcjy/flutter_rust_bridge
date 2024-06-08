use quote::ToTokens;

pub(crate) fn canonicalize_rust_type(raw: &str) -> anyhow::Result<String> {
    let ast: syn::Type = syn::parse_str(raw)?;
    Ok(quote::quote!(#ast).to_string())
}

// TODO rename
pub(crate) fn ty_to_string<T: ToTokens>(ty: &T) -> String {
    quote::quote!(#ty).to_string()
}

pub(crate) fn parse_attribute(s: &str) -> anyhow::Result<syn::Attribute> {
    let ast: syn::ItemMod = syn::parse_str(&format!("{s} mod m {{}}"))?;
    assert_eq!(ast.attrs.len(), 1);
    Ok(ast.attrs.into_iter().next().unwrap())
}
