use syn::*;

fn add_mirrored(mirrors: &str, paths: &mut Vec<Path>) {
    if let Some(mirrored) = mirrors
        .replace(' ', "")
        .strip_prefix("mirror(")
        .and_then(|c| c.strip_suffix(')'))
    {
        let mirror_paths = mirrored
            .split(',')
            .map(|p| syn::parse_str::<Path>(p).unwrap());
        paths.extend(mirror_paths);
    }
}

/// Extract a path from marker `#[frb(mirror(path), ..)]`
pub fn extract_mirror_marker(attrs: &[Attribute]) -> Vec<Path> {
    let mut paths = vec![];
    attrs.iter().for_each(|attr| {
        // case with expanded frb macros
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }),
                ..
            }) = &attr.meta
            {
                add_mirrored(&lit.value(), &mut paths);
            }
        }
        // case with not expanded frb macros
        if attr.path().is_ident("frb") {
            if let Meta::List(list) = &attr.meta {
                add_mirrored(&list.tokens.to_string(), &mut paths);
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
