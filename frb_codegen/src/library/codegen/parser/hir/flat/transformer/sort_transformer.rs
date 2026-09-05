use crate::codegen::ir::hir::flat::component::HirFlatComponent;
use crate::codegen::ir::hir::flat::pack::{HirFlatPack, HirFlatPackComponentVisitor};

pub(crate) fn transform(mut pack: HirFlatPack) -> anyhow::Result<HirFlatPack> {
    sort_hir_flat_pack(&mut pack);
    Ok(pack)
}

pub(crate) fn sort_hir_flat_pack(pack: &mut HirFlatPack) {
    pack.visit_components_mut(Visitor);
}

struct Visitor;

impl HirFlatPackComponentVisitor for Visitor {
    fn visit<SK: Ord, T: HirFlatComponent<SK>>(&self, items: &mut Vec<T>) {
        items.sort_by_cached_key(|item| item.sort_key());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::hir::flat::type_alias::HirFlatTypeAlias;

    /// Sorts type aliases by their component-specific identifier keys.
    #[test]
    fn sorts_type_aliases_by_identifier() {
        let mut pack = HirFlatPack {
            types: vec![
                HirFlatTypeAlias {
                    ident: "Zebra".to_owned(),
                    target: syn::parse_str("i32").unwrap(),
                    type_params: vec![],
                },
                HirFlatTypeAlias {
                    ident: "Apple".to_owned(),
                    target: syn::parse_str("u32").unwrap(),
                    type_params: vec![],
                },
            ],
            ..HirFlatPack::default()
        };

        sort_hir_flat_pack(&mut pack);

        assert_eq!(
            pack.types
                .iter()
                .map(|alias| &alias.ident)
                .collect::<Vec<_>>(),
            ["Apple", "Zebra"]
        );
    }
}
