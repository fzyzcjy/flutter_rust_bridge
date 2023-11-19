use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::impl_new_with_nullptr::generate_impl_new_with_nullptr;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::impl_wire2api_trait::generate_impl_wire2api;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::library::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;

mod impl_new_with_nullptr;
mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

pub(crate) struct WireRustOutputSpecWire2api {
    pub allocate_funcs: Acc<Vec<WireRustOutputCode>>,
    pub related_funcs: Acc<Vec<WireRustOutputCode>>,
    pub impl_wire2api: Acc<Vec<WireRustOutputCode>>,
    pub wire2api_class: Acc<Vec<WireRustOutputCode>>,
    pub impl_new_with_nullptr: Acc<Vec<WireRustOutputCode>>,
}

pub(crate) fn generate(
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireRustOutputSpecWire2api {
    WireRustOutputSpecWire2api {
        allocate_funcs: cache
            .distinct_input_types
            .iter()
            .map(|ty| WireRustGenerator::new(ty.clone(), context).generate_allocate_funcs())
            .collect(),
        related_funcs: cache
            .distinct_output_types
            .iter()
            .map(|ty| WireRustGenerator::new(ty.clone(), context).generate_related_funcs())
            .collect(),
        impl_wire2api: generate_impl_wire2api(&cache.distinct_input_types, context),
        wire2api_class: Acc::new_io(
            cache
                .distinct_input_types
                .iter()
                .filter_map(|ty| {
                    WireRustGenerator::new(ty.clone(), context).generate_wire2api_class()
                })
                .map(|x| x.into())
                .collect(),
        ),
        impl_new_with_nullptr: Acc::new_io(generate_impl_new_with_nullptr(
            &cache.distinct_input_types,
            context,
        )),
    }
}

// TODO rm, since no longer have explicit SyncReturn type?
// TODO -> no, this is needed
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
