use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::base::WireRustGeneratorContext;
use crate::codegen::ir::pack::IrPack;

pub(super) mod extern_func;
mod impl_new_with_nullptr;
mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

pub(crate) fn generate(ir_pack: &IrPack, context: WireRustGeneratorContext) -> Acc<Vec<String>> {
    let mut lines = Acc::<Vec<_>>::default();
    todo!();
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
