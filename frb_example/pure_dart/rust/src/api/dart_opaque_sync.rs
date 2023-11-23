// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

use flutter_rust_bridge::{frb, DartOpaque};

/// [DartWrapObject] can be safely retrieved on a dart thread.
#[frb(sync)]
pub fn unwrap_dart_opaque(opaque: DartOpaque) -> String {
    let handle = opaque.try_unwrap().unwrap();
    "Test".to_owned()
}

#[frb(sync)]
pub fn return_non_droppable_dart_opaque(opaque: DartOpaque) -> DartOpaque {
    let raw = opaque.try_unwrap().unwrap();
    unsafe { DartOpaque::new_non_droppable(raw.into()) }
}
