use crate::codegen::ir::hir::hierarchical::module::HirModule;

pub(super) fn transform(items: &[syn::Item]) -> anyhow::Result<Vec<syn::Item>> {
    let items = items.to_owned();
    for item in items.iter_mut() {
        TODO;
    }
    Ok(items)
}
