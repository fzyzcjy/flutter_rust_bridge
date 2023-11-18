mod extern_func;
mod impl_new_with_nullptr;
mod impl_wire2api_trait;
mod misc;
pub(crate) mod ty;

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
