// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_sync.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse", "sync moi", "rustAsync moi", "sync sse moi", "rustAsync sse moi"], "enableAll": true}

use super::rust_opaque_twin_sse::NonCloneDataTwinSse;
pub use crate::auxiliary::sample_types::{HideDataRaw, NonCloneDataRaw};
#[allow(unused_imports)]
use crate::frb_generated::RustOpaque;
use anyhow::Result;
use flutter_rust_bridge::frb;

#[allow(dead_code)]
pub struct HideDataAnotherTwinSse(HideDataRaw);

/// Structure for testing the SyncReturn<RustOpaque> code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must should be without wrapper like Option<> Vec<> etc.
pub struct FrbOpaqueSyncReturnTwinSse;

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn sync_option_rust_opaque_twin_sse() -> Result<Option<RustOpaque<HideDataAnotherTwinSse>>> {
    Ok(Some(RustOpaque::new(HideDataAnotherTwinSse(
        HideDataRaw::new(),
    ))))
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn sync_create_opaque_twin_sse() -> RustOpaque<HideDataAnotherTwinSse> {
    RustOpaque::new(HideDataAnotherTwinSse(HideDataRaw::new()))
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn sync_create_non_clone_twin_sse() -> RustOpaque<NonCloneDataTwinSse> {
    RustOpaque::new(NonCloneDataTwinSse(NonCloneDataRaw::new()))
}

// OpaqueSyncStruct does not implement Send trait.
//
// #[flutter_rust_bridge::frb(serialize)] pub fn run_opaque_twin_sse(opaque: Opaque<OpaqueSyncStruct>) -> String {
//     data.0.hide_data()
// }

/// Structure for testing the sync-mode RustOpaque code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must be without wrapper like Option<> Vec<> etc.
#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn frb_sync_generator_test_twin_sse() -> RustOpaque<FrbOpaqueSyncReturnTwinSse> {
    RustOpaque::new(FrbOpaqueSyncReturnTwinSse)
}
