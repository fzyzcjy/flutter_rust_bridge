// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `newtype_pattern.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use log::info;

#[derive(Debug)]
pub struct NewTypeIntTwinSse(pub i64);

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_newtype_twin_sse(arg: NewTypeIntTwinSse) -> NewTypeIntTwinSse {
    info!("handle_newtype({:?})", &arg);
    NewTypeIntTwinSse(arg.0 * 2)
}

// TODO use auto generated sync code instead
// #[flutter_rust_bridge::frb(serialize)] pub fn handle_newtype_sync_twin_sse(arg: NewTypeInt) -> SyncReturn<NewTypeInt> {
//     info!("handle_newtype_sync({:?})", &arg);
//     SyncReturn(NewTypeInt(arg.0 * 2))
// }
