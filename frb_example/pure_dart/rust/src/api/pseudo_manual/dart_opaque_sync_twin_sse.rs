// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_opaque_sync.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::{frb, DartOpaque};

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn sync_loopback_twin_sse(opaque: DartOpaque) -> DartOpaque {
    opaque
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn sync_option_loopback_twin_sse(opaque: Option<DartOpaque>) -> Option<DartOpaque> {
    opaque
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn sync_accept_dart_opaque_twin_sse(opaque: DartOpaque) -> String {
    drop(opaque);
    "test".to_owned()
}

/// [DartWrapObject] can be safely retrieved on a dart thread.
#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn unwrap_dart_opaque_twin_sse(opaque: DartOpaque) -> String {
    let _handle = opaque.into_inner().unwrap();
    "Test".to_owned()
}

// This API is removed in v2, because we can always provide the drop port
// #[frb(sync)]
// #[flutter_rust_bridge::frb(serialize)] pub fn return_non_droppable_dart_opaque_twin_sse(opaque: DartOpaque) -> DartOpaque {
//     let raw = opaque.try_unwrap().unwrap();
//     unsafe { DartOpaque::new_non_droppable(raw.into()) }
// }

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn sync_option_dart_opaque_twin_sse(opaque: DartOpaque) -> anyhow::Result<Option<DartOpaque>> {
    Ok(Some(opaque))
}
