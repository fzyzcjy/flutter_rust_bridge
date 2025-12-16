use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;
use syn::ItemType;

pub(crate) fn parse_syn_item_type(item_type: ItemType) -> Option<HirFlatTypeAlias> {
    // debug!("parse_syn_item_type item_type={item_type:?}");
    let has_generics =
        item_type.generics.where_clause.is_some() || item_type.generics.lt_token.is_some();
    Some(HirFlatTypeAlias {
        ident: item_type.ident.to_string(),
        target: *item_type.ty,
        generics: if has_generics {
            Some(item_type.generics)
        } else {
            None
        },
    })
}
