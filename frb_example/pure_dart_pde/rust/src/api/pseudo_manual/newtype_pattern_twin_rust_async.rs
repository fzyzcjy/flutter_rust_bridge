// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `newtype_pattern.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use log::info;

#[derive(Debug)]
pub struct NewTypeIntTwinRustAsync(pub i64);

pub async fn handle_newtype_twin_rust_async(
    arg: NewTypeIntTwinRustAsync,
) -> NewTypeIntTwinRustAsync {
    info!("handle_newtype({:?})", &arg);
    NewTypeIntTwinRustAsync(arg.0 * 2)
}

// TODO use auto generated sync code instead
// pub async fn handle_newtype_sync_twin_rust_async(arg: NewTypeInt) -> SyncReturn<NewTypeInt> {
//     info!("handle_newtype_sync({:?})", &arg);
//     SyncReturn(NewTypeInt(arg.0 * 2))
// }
