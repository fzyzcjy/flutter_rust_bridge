use syn::*;

/// Extract a path from marker `#[frb(mirror(path), ..)]`
pub fn extract_mirror_marker(attrs: &[Attribute]) -> Vec<Path> {
    let mut paths = vec![];
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .for_each(|attr| {
            if let Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ..
            }) = &attr.meta
            {
                let comment = lit.value();
                if let Some(mirrored) = comment
                    .trim()
                    .strip_prefix("mirror(")
                    .and_then(|c| c.strip_suffix(')'))
                {
                    let mirror_paths = mirrored
                        .split(',')
                        .map(|p| syn::parse_str::<Path>(p).unwrap());
                    paths.extend(mirror_paths);
                }
            }
        });
    paths
}

/// Checks if the `#[frb(non_final)]` attribute is present.
pub fn has_non_final(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("frb"))
        .any(|attr| {
            let mut flag = false;
            let _ = attr.parse_nested_meta(|arg| {
                if arg.path.is_ident("non_final") {
                    flag = true;
                }
                Ok(())
            });
            flag
        })
}
