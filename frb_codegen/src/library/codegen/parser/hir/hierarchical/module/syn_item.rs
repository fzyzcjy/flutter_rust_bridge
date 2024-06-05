use crate::codegen::ir::hir::hierarchical::module::{
    HirModule, HirModuleContent, HirModuleMeta, HirVisibility,
};
use crate::codegen::parser::hir::hierarchical::function::{parse_syn_item_fn, parse_syn_item_impl};
use crate::codegen::parser::hir::hierarchical::item_type::parse_syn_item_type;
use crate::codegen::parser::hir::hierarchical::module::parse_module;
use crate::codegen::parser::hir::hierarchical::struct_or_enum::{
    parse_syn_item_enum, parse_syn_item_struct,
};
use crate::codegen::parser::hir::hierarchical::traits::{parse_syn_item_trait, parse_trait_impl};
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::utils::namespace::Namespace;
use syn::ItemMod;

// moved
// fn parse_syn_item_mod(
//     item_mod: &ItemMod,
//     namespace: &Namespace,
//     config: &ParserHirInternalConfig,
//     parent_vis: &[HirVisibility],
// ) -> anyhow::Result<Option<HirModule>> {
//     Ok(if let Some((_, items)) = &item_mod.content {
//         let info = HirModuleMeta {
//             parent_vis: parent_vis.to_owned(),
//             vis: (&item_mod.vis).into(),
//             namespace: namespace.join(&item_mod.ident.to_string()),
//         };
//         Some(parse_module(
//             items, info,
//             config,
//             // cumulated_visibility_pub && matches!(item_mod.vis, syn::Visibility::Public(_)),
//         )?)
//     } else {
//         None
//     })
// }
