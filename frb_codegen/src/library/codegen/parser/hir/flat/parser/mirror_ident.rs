use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use anyhow::Context;
use itertools::Itertools;
use proc_macro2::Ident;
use syn::{Attribute, PathArguments};

pub(crate) struct ParseMirrorIdentOutput {
    pub idents: Vec<Ident>,
    pub mirror: bool,
}

/// Get a struct or enum ident, possibly remapped by a mirror marker
pub(crate) fn parse_mirror_ident(
    ident: &Ident,
    attrs: &[Attribute],
) -> anyhow::Result<ParseMirrorIdentOutput> {
    let attributes = FrbAttributes::parse(attrs)
        .with_context(|| format!("when parsing ident={ident:?} attrs={attrs:?}"))?;
    let mirror_info = attributes.mirror();

    let res = mirror_info
        .into_iter()
        .filter_map(|path| {
            if path.leading_colon.is_none()
                && path.segments.len() == 1
                && path.segments[0].arguments == PathArguments::None
            {
                Some(path.segments.into_iter().next().unwrap().ident)
            } else {
                None
            }
        })
        .collect_vec();

    let mirror = !res.is_empty();

    Ok(ParseMirrorIdentOutput {
        idents: if mirror { res } else { vec![ident.clone()] },
        mirror,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_mirror_ident;
    use quote::format_ident;

    /// Preserves the original identifier when no mirror marker exists.
    #[test]
    fn preserves_ident_without_mirror_attribute() -> anyhow::Result<()> {
        let output = parse_mirror_ident(&format_ident!("Original"), &[])?;

        assert!(!output.mirror);
        assert_eq!(output.idents[0], format_ident!("Original"));
        Ok(())
    }

    /// Uses each simple mirror identifier from the FRB attribute.
    #[test]
    fn parses_simple_mirror_identifiers() -> anyhow::Result<()> {
        let attrs = vec![syn::parse_quote!(
            #[flutter_rust_bridge::frb(mirror(First, Second))]
        )];
        let output = parse_mirror_ident(&format_ident!("Original"), &attrs)?;

        assert!(output.mirror);
        assert_eq!(
            output.idents,
            vec![format_ident!("First"), format_ident!("Second")]
        );
        Ok(())
    }
}
