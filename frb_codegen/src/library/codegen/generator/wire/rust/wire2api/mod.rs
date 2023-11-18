use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::misc::section_header_comment;
use crate::codegen::generator::wire::rust::wire2api::impl_new_with_nullptr::generate_impl_new_with_nullptr;
use crate::codegen::generator::wire::rust::wire2api::impl_wire2api_trait::generate_impl_wire2api;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;

pub(super) mod extern_func;
mod impl_new_with_nullptr;
mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> Acc<Vec<String>> {
    let mut lines = Acc::<Vec<_>>::default();

    lines.push(section_header_comment("allocate functions"));
    lines += cache
        .distinct_input_types
        .iter()
        .map(|ty| WireRustGenerator::new(ty.clone(), context).generate_allocate_funcs())
        .collect();

    lines.push(section_header_comment("related functions"));
    lines += cache
        .distinct_output_types
        .iter()
        .map(|ty| WireRustGenerator::new(ty.clone(), context).generate_related_funcs())
        .collect();

    lines.push(section_header_comment("impl Wire2Api"));
    lines += generate_impl_wire2api(&cache.distinct_input_types, context);

    lines.io.push(section_header_comment("wire structs"));
    lines.io.extend(
        cache
            .distinct_input_types
            .iter()
            .filter_map(|ty| WireRustGenerator::new(ty.clone(), context).generate_wire2api_class()),
    );

    (lines.io).push(section_header_comment("impl NewWithNullPtr"));
    (lines.io).push(generate_impl_new_with_nullptr(
        &cache.distinct_input_types,
        context,
    ));

    lines
}

// TODO rm, since no longer have explicit SyncReturn type?
// pub(crate) fn generate_sync_execution_mode_utility() -> CodeWithExternFunc {
//     CodeWithExternFunc {
//         extern_funcs: vec![ExternFunc {
//             func_name: "free_WireSyncReturn".to_owned(),
//             params: vec![ExternFuncParam {
//                 name: "ptr".to_owned(),
//                 rust_type: "support::WireSyncReturn".to_owned(),
//                 dart_type: None,
//             }],
//             return_type: None,
//             body: "unsafe { let _ = support::box_from_leak_ptr(ptr); };".to_owned(),
//             target: Target::Io,
//         }],
//         ..Default::default()
//     }
// }
