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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Canonicalizes equivalent Rust type spellings through syn and quote.
    fn canonicalizes_rust_type() -> anyhow::Result<()> {
        assert_eq!(
            canonicalize_rust_type("Option<Result<String,u8>>")?,
            "Option < Result < String , u8 > >"
        );

        Ok(())
    }

    #[test]
    /// Rejects invalid Rust type syntax.
    fn canonicalize_rust_type_rejects_invalid_syntax() {
        assert!(canonicalize_rust_type("Vec<").is_err());
    }

    #[test]
    /// Converts tokenizable syntax nodes into quote's canonical token string.
    fn converts_tokens_to_a_string() {
        let ty: syn::Type = syn::parse_str("Option<Result<String, u8>>").unwrap();

        assert_eq!(ty_to_string(&ty), "Option < Result < String , u8 > >");
    }

    #[test]
    /// Parses a single outer attribute while preserving its tokens.
    fn parses_attribute() -> anyhow::Result<()> {
        let attribute = parse_attribute("#[frb(sync)]")?;

        assert_eq!(attribute.to_token_stream().to_string(), "# [frb (sync)]");

        Ok(())
    }

    #[test]
    /// Rejects strings that do not form an outer attribute.
    fn parse_attribute_rejects_non_attributes() {
        assert!(parse_attribute("not an attribute").is_err());
    }
}
