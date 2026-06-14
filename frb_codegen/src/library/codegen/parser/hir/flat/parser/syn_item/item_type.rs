use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;
use syn::ItemType;

pub(crate) fn parse_syn_item_type(item_type: ItemType) -> Option<HirFlatTypeAlias> {
    // debug!("parse_syn_item_type item_type={item_type:?}");

    // Aliases with a `where` clause are not supported yet.
    if item_type.generics.where_clause.is_some() {
        return None;
    }

    let type_params: Vec<String> = (item_type.generics.type_params())
        .map(|param| param.ident.to_string())
        .collect();

    // Only generic aliases whose parameters are *all* type params are supported
    // (e.g. `type AppResult<T> = Result<T, AppError>`). Aliases that also carry
    // lifetime or const generics (e.g. `type Fixed<T, const N: usize> = [T; N]`)
    // are skipped: substitution neither records nor replaces those params, so
    // expanding such an alias would silently drop information and emit wrong
    // code. Lifetime-only / const-only aliases are skipped for the same reason.
    let all_params_are_type_params = type_params.len() == item_type.generics.params.len();
    if !all_params_are_type_params {
        return None;
    }

    Some(HirFlatTypeAlias {
        ident: item_type.ident.to_string(),
        target: *item_type.ty,
        type_params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn keeps_non_generic_alias() {
        let alias = parse_syn_item_type(parse_quote!(
            pub type Id = u64;
        ))
        .unwrap();
        assert_eq!(alias.ident, "Id");
        assert!(alias.type_params.is_empty());
    }

    #[test]
    fn keeps_type_param_only_alias() {
        let alias = parse_syn_item_type(parse_quote!(
            pub type AppResult<T> = Result<T, AppError>;
        ))
        .unwrap();
        assert_eq!(alias.ident, "AppResult");
        assert_eq!(alias.type_params, vec!["T".to_owned()]);
    }

    #[test]
    fn skips_alias_with_const_generic() {
        // `N` would be silently dropped during substitution, so it must be skipped.
        assert!(parse_syn_item_type(parse_quote!(
            pub type Fixed<T, const N: usize> = [T; N];
        ))
        .is_none());
    }

    #[test]
    fn skips_alias_with_lifetime() {
        assert!(parse_syn_item_type(parse_quote!(
            pub type Borrowed<'a, T> = &'a T;
        ))
        .is_none());
    }

    #[test]
    fn skips_alias_with_where_clause() {
        // `where` clauses are not supported yet, so such aliases are skipped.
        assert!(parse_syn_item_type(parse_quote!(
            pub type Aliased<T>
            where
                T: Clone,
            = Vec<T>;
        ))
        .is_none());
    }
}
