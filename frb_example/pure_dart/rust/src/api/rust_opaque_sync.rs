// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

use crate::api::rust_opaque::HideData;
use crate::auxiliary::sample_types::{FrbOpaqueSyncReturn, NonCloneData, NonSendHideData};
use anyhow::Result;
use flutter_rust_bridge::{frb, RustOpaque};

#[frb(sync)]
pub fn sync_option_rust_opaque() -> Result<Option<RustOpaque<HideData>>> {
    Ok(Some(RustOpaque::new(HideData::new())))
}

#[frb(sync)]
pub fn sync_create_opaque() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

#[frb(sync)]
pub fn sync_create_sync_opaque() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

#[frb(sync)]
pub fn sync_create_non_clone() -> NonCloneData {
    NonCloneData::new()
}

// OpaqueSyncStruct does not implement Send trait.
//
// pub fn run_opaque(opaque: Opaque<OpaqueSyncStruct>) -> String {
//     data.0.hide_data()
// }

#[frb(sync)]
pub fn sync_run_opaque(opaque: RustOpaque<NonSendHideData>) -> String {
    opaque.hide_data()
}

/// Structure for testing the sync-mode RustOpaque code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must be without wrapper like Option<> Vec<> etc.
#[frb(sync)]
pub fn frb_sync_generator_test() -> RustOpaque<FrbOpaqueSyncReturn> {
    RustOpaque::new(FrbOpaqueSyncReturn)
}
