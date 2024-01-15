// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `newtype_pattern.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use log::info;

#[derive(Debug)]
pub struct NewTypeIntTwinSync(pub i64);

#[flutter_rust_bridge::frb(sync)]
pub fn handle_newtype_twin_sync(arg: NewTypeIntTwinSync) -> NewTypeIntTwinSync {
    info!("handle_newtype({:?})", &arg);
    NewTypeIntTwinSync(arg.0 * 2)
}

// TODO use auto generated sync code instead
// #[flutter_rust_bridge::frb(sync)] pub fn handle_newtype_sync_twin_sync(arg: NewTypeInt) -> SyncReturn<NewTypeInt> {
//     info!("handle_newtype_sync({:?})", &arg);
//     SyncReturn(NewTypeInt(arg.0 * 2))
// }
