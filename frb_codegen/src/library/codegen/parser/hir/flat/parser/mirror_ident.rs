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
