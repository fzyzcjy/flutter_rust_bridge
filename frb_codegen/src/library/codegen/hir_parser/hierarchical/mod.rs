use crate::codegen::hir::hierarchical::crates::HirCrate;
use crate::codegen::hir::hierarchical::module::{HirModuleInfo, HirVisibility};
use crate::codegen::hir_parser::hierarchical::module::parse_module;
use crate::codegen::ir::namespace::Namespace;

pub(crate) mod item_type;
pub(crate) mod mirror_ident;
pub(crate) mod module;
pub(crate) mod struct_or_enum;
pub(crate) mod visibility;

pub(crate) fn parse(file: syn::File) -> anyhow::Result<HirCrate> {
    let info = HirModuleInfo {
        visibility: HirVisibility::Public,
        namespace: Namespace::new(vec![Namespace::SELF_CRATE.to_owned()]),
    };
    let root_module = parse_module(&file.items, info)?;
    Ok(HirCrate { root_module })
}
