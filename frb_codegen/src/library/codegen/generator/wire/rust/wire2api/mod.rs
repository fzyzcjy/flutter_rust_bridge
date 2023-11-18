use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::misc::section_header_comment;
use crate::codegen::generator::wire::rust::wire2api::impl_wire2api_trait::generate_impl_wire2api;
use crate::codegen::generator::wire::rust::IrPackComputedCache;
use crate::codegen::ir::pack::IrPack;

pub(super) mod extern_func;
mod impl_new_with_nullptr;
mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
    cache: &IrPackComputedCache,
) -> Acc<Vec<String>> {
    let mut lines = Acc::<Vec<_>>::default();

    lines.push(section_header_comment("allocate functions"));
    lines += cache
        .distinct_input_types
        .iter()
        .map(|ty| WireRustGenerator::new(ty, context).generate_allocate_funcs())
        .collect();

    lines.push(section_header_comment("related functions"));
    lines += cache
        .distinct_output_types
        .iter()
        .map(|ty| WireRustGenerator::new(ty, context).generate_related_funcs())
        .collect();

    lines.push(section_header_comment("impl Wire2Api"));
    lines += generate_impl_wire2api(&cache.distinct_input_types, context);

    lines.io.push(self.section_header_comment("wire structs"));
    // TODO these become: wire::rust::class::generate
    // lines.io.extend(
    //     distinct_input_types
    //         .iter()
    //         .map(|ty| self.generate_wire_struct(ty, ir_pack)),
    // );
    // lines.io.extend(
    //     distinct_input_types
    //         .iter()
    //         .map(|ty| TypeRustGenerator::new(ty.clone(), ir_pack, self.config).structs()),
    // );

    (lines.io).push(self.section_header_comment("impl NewWithNullPtr"));
    // TODO
    // (lines.io).push(self.generate_new_with_nullptr_misc().to_string());
    // lines.io.extend(
    //     distinct_input_types
    //         .iter()
    //         .map(|ty| self.generate_new_with_nullptr_func(ty, ir_pack)),
    // );

    // moved
    // (lines.wasm).push(self.section_header_comment("impl Wire2Api for JsValue"));
    // (lines.wasm).push(
    //     "impl<T> Wire2Api<Option<T>> for JsValue where JsValue: Wire2Api<T> {
    //         fn wire2api(self) -> Option<T> {
    //             (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    //         }
    //     }"
    //     .into(),
    // );

    // TODO moved
    // lines.wasm.extend(
    //     distinct_input_types
    //         .iter()
    //         .filter_map(|ty| self.generate_wasm2api_func(ty, ir_pack)),
    // );

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
