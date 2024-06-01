use crate::codegen::hir::hierarchical::crates::HirCrate;
use crate::codegen::hir::hierarchical::module::HirVisibility;
use crate::codegen::hir_parser::hierarchical::module::parse_module;

pub(crate) mod item_type;
pub(crate) mod mirror_ident;
pub(crate) mod module;
pub(crate) mod struct_or_enum;
pub(crate) mod visibility;

pub(crate) fn parse(file: syn::File) -> anyhow::Result<HirCrate> {
    let root_modulee = parse_module(&file.items, HirVisibility::Public)?;
    Ok(HirCrate { root_module })
}
