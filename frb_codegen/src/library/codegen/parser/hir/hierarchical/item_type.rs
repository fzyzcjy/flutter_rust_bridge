use crate::codegen::ir::hir::hierarchical::type_alias::HirTypeAlias;
use syn::ItemType;

pub(crate) fn parse_syn_item_type(item_type: &ItemType) -> Option<HirTypeAlias> {
    // debug!("parse_syn_item_type item_type={item_type:?}");
    if item_type.generics.where_clause.is_none() && item_type.generics.lt_token.is_none() {
        Some(HirTypeAlias {
            ident: item_type.ident.to_string(),
            target: *item_type.ty.clone(),
        })
    } else {
        None
    }
}
