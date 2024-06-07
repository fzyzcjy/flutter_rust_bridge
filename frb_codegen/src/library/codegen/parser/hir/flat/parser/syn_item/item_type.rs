use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;
use syn::ItemType;

pub(crate) fn parse_syn_item_type(item_type: ItemType) -> Option<HirFlatTypeAlias> {
    // debug!("parse_syn_item_type item_type={item_type:?}");
    if item_type.generics.where_clause.is_none() && item_type.generics.lt_token.is_none() {
        Some(HirFlatTypeAlias {
            ident: item_type.ident.to_string(),
            target: *item_type.ty,
        })
    } else {
        None
    }
}
