// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::{frb, DartOpaque};

#[frb(sync)]
pub fn sync_loopback_twin_normal(opaque: DartOpaque) -> DartOpaque {
    opaque
}

#[frb(sync)]
pub fn sync_option_loopback_twin_normal(opaque: Option<DartOpaque>) -> Option<DartOpaque> {
    opaque
}

#[frb(sync)]
pub fn sync_accept_dart_opaque_twin_normal(opaque: DartOpaque) -> String {
    drop(opaque);
    "test".to_owned()
}

/// [DartWrapObject] can be safely retrieved on a dart thread.
#[frb(sync)]
pub fn unwrap_dart_opaque_twin_normal(opaque: DartOpaque) -> String {
    let _handle = opaque.into_inner().unwrap();
    "Test".to_owned()
}

// This API is removed in v2, because we can always provide the drop port
// #[frb(sync)]
// pub fn return_non_droppable_dart_opaque_twin_normal(opaque: DartOpaque) -> DartOpaque {
//     let raw = opaque.try_unwrap().unwrap();
//     unsafe { DartOpaque::new_non_droppable(raw.into()) }
// }

#[frb(sync)]
pub fn sync_option_dart_opaque_twin_normal(
    opaque: DartOpaque,
) -> anyhow::Result<Option<DartOpaque>> {
    Ok(Some(opaque))
}
