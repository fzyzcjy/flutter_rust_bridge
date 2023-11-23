use anyhow::Result;
use flutter_rust_bridge::support::lazy_static;
use flutter_rust_bridge::{frb, DartOpaque};

// TODO about sync
// pub fn sync_accept_dart_opaque(opaque: DartOpaque) -> SyncReturn<String> {
//     drop(opaque);
//     SyncReturn("test".to_owned())
// }

pub fn async_accept_dart_opaque(opaque: DartOpaque) -> String {
    drop(opaque);
    "async test".to_owned()
}

pub fn loop_back(opaque: DartOpaque) -> DartOpaque {
    opaque
}

pub fn loop_back_option(opaque: DartOpaque) -> Option<DartOpaque> {
    Some(opaque)
}

pub fn loop_back_array(opaque: DartOpaque) -> [DartOpaque; 1] {
    [opaque]
}

pub fn loop_back_vec(opaque: DartOpaque) -> Vec<DartOpaque> {
    vec![opaque]
}

pub fn loop_back_option_get(opaque: Option<DartOpaque>) {}

pub fn loop_back_array_get(opaque: [DartOpaque; 1]) {}

pub fn loop_back_vec_get(opaque: Vec<DartOpaque>) {}

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
pub fn panic_unwrap_dart_opaque(opaque: DartOpaque) {
    let handle = opaque.try_unwrap().unwrap();
}

pub enum EnumDartOpaque {
    Primitive(i32),
    Opaque(DartOpaque),
}

pub struct DartOpaqueNested {
    pub first: DartOpaque,
    pub second: DartOpaque,
}

// TODO about sync
// pub fn sync_loopback(opaque: DartOpaque) -> SyncReturn<DartOpaque> {
//     SyncReturn(opaque)
// }
//
// pub fn sync_option_loopback(opaque: Option<DartOpaque>) -> SyncReturn<Option<DartOpaque>> {
//     SyncReturn(opaque)
// }
//
// pub fn sync_option_dart_opaque(opaque: DartOpaque) -> Result<SyncReturn<Option<DartOpaque>>> {
//     Ok(SyncReturn(Some(opaque)))
// }

pub fn create_nested_dart_opaque(opaque1: DartOpaque, opaque2: DartOpaque) -> DartOpaqueNested {
    DartOpaqueNested {
        first: opaque1,
        second: opaque2,
    }
}

pub fn get_nested_dart_opaque(opaque: DartOpaqueNested) {}

pub fn create_enum_dart_opaque(opaque: DartOpaque) -> EnumDartOpaque {
    EnumDartOpaque::Opaque(opaque)
}

pub fn get_enum_dart_opaque(opaque: EnumDartOpaque) {}

lazy_static! {
    static ref DART_OPAQUE: Mutex<Option<DartOpaque>> = Default::default();
}

pub fn set_static_dart_opaque(opaque: DartOpaque) {
    *DART_OPAQUE.lock().unwrap() = Some(opaque);
}

pub fn drop_static_dart_opaque() {
    drop(DART_OPAQUE.lock().unwrap().take());
}
