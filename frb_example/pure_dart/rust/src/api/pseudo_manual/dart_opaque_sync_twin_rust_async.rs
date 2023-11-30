// NOTE: This file is mimicking how a human developer writes tests, 
// and is auto-generated from `dart_opaque_sync.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

use flutter_rust_bridge::{frb, DartOpaque};

#[frb(sync)]
pub async fn sync_loopback_twin_rust_async(opaque: DartOpaque) -> DartOpaque {
    opaque
}

#[frb(sync)]
pub async fn sync_option_loopback_twin_rust_async(opaque: Option<DartOpaque>) -> Option<DartOpaque> {
    opaque
}

#[frb(sync)]
pub async fn sync_accept_dart_opaque_twin_rust_async(opaque: DartOpaque) -> String {
    drop(opaque);
    "test".to_owned()
}

/// [DartWrapObject] can be safely retrieved on a dart thread.
#[frb(sync)]
pub async fn unwrap_dart_opaque_twin_rust_async(opaque: DartOpaque) -> String {
    let handle = opaque.try_unwrap().unwrap();
    "Test".to_owned()
}

#[frb(sync)]
pub async fn return_non_droppable_dart_opaque_twin_rust_async(opaque: DartOpaque) -> DartOpaque {
    let raw = opaque.try_unwrap().unwrap();
    unsafe { DartOpaque::new_non_droppable(raw.into()) }
}

#[frb(sync)]
pub async fn sync_option_dart_opaque_twin_rust_async(
    opaque: DartOpaque,
) -> anyhow::Result<Option<DartOpaque>> {
    Ok(Some(opaque))
}
