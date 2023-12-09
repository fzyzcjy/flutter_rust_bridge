use crate::codec::dco::Rust2DartMessageDco;
use crate::codec::sse::Rust2DartMessageSse;
use crate::codec::Rust2DartMessageTrait;
use crate::platform_types::{WireSyncReturnDco, WireSyncReturnSse};
pub use allo_isolate::*;
use dart_sys::Dart_InitializeApiDL;

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn init_frb_dart_api_dl(data: *mut std::ffi::c_void) -> isize {
    Dart_InitializeApiDL(data)
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn free_wire_sync_return_dco(value: WireSyncReturnDco) {
    let _ = Rust2DartMessageDco::from_raw_wire_sync(value);
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn free_wire_sync_return_sse(value: WireSyncReturnSse) {
    let _ = Rust2DartMessageSse::from_raw_wire_sync(value);
}
