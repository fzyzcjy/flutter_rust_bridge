use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;
use syn::ItemType;

pub(crate) fn parse_syn_item_type(item_type: ItemType) -> Option<HirFlatTypeAlias> {
    // debug!("parse_syn_item_type item_type={item_type:?}");

    // v1: aliases with a `where` clause are not yet supported.
    if item_type.generics.where_clause.is_some() {
        return None;
    }

    let type_params: Vec<String> = (item_type.generics.type_params())
        .map(|param| param.ident.to_string())
        .collect();

    // Keep non-generic aliases (existing behavior) and generic aliases that
    // introduce type parameters (e.g. `type AppResult<T> = Result<T, AppError>`).
    // Generic aliases without any type parameter (e.g. lifetime-only) remain
    // unsupported, preserving the previous behavior.
    if item_type.generics.lt_token.is_some() && type_params.is_empty() {
        return None;
    }

    Some(HirFlatTypeAlias {
        ident: item_type.ident.to_string(),
        target: *item_type.ty,
        type_params,
    })
}
