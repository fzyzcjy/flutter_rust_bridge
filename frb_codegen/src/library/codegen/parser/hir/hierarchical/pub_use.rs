use crate::codegen::ir::hir::hierarchical::module::HirModule;

pub(crate) fn transform_module_by_pub_use(
    raw: HirModule,
    items: &[syn::Item],
) -> anyhow::Result<HirModule> {
    TODO
}

fn parse_pub_use(items: &[syn::Item]) -> Vec<TODO> {
    TODO
}
