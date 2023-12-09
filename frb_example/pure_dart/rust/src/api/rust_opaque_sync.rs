// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

use crate::api::rust_opaque::HideData;
use crate::auxiliary::sample_types::{FrbOpaqueSyncReturn, NonCloneData, NonSendHideData};
use anyhow::Result;
use flutter_rust_bridge::{frb, RustOpaque};

#[frb(sync)]
pub fn sync_option_rust_opaque_twin_normal() -> Result<Option<RustOpaque<HideData>>> {
    Ok(Some(RustOpaque::new(HideData::new())))
}

#[frb(sync)]
pub fn sync_create_opaque_twin_normal() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

#[frb(sync)]
pub fn sync_create_sync_opaque_twin_normal() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

#[frb(sync)]
pub fn sync_create_non_clone_twin_normal() -> RustOpaque<NonCloneData> {
    RustOpaque::new(NonCloneData::new())
}

// OpaqueSyncStruct does not implement Send trait.
//
// pub fn run_opaque_twin_normal(opaque: Opaque<OpaqueSyncStruct>) -> String {
//     data.0.hide_data()
// }

#[frb(sync)]
pub fn sync_run_opaque_twin_normal(opaque: RustOpaque<NonSendHideData>) -> String {
    opaque.hide_data()
}

/// Structure for testing the sync-mode RustOpaque code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must be without wrapper like Option<> Vec<> etc.
#[frb(sync)]
pub fn frb_sync_generator_test_twin_normal() -> RustOpaque<FrbOpaqueSyncReturn> {
    RustOpaque::new(FrbOpaqueSyncReturn)
}
