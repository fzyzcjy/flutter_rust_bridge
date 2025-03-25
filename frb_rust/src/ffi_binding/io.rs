use crate::codec::dco::Rust2DartMessageDco;
use crate::codec::sse::Rust2DartMessageSse;
use crate::codec::Rust2DartMessageTrait;
use crate::platform_types::{WireSyncRust2DartDco, WireSyncRust2DartSse};
use allo_isolate::{
    ffi::{DartCObject, DartPort},
    store_dart_post_cobject,
};
use std::ffi::c_void;

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_init_frb_dart_api_dl(data: *mut std::ffi::c_void) -> isize {
    #[cfg(feature = "dart-opaque")]
    return dart_sys::Dart_InitializeApiDL(data);
    #[cfg(not(feature = "dart-opaque"))]
    return 0;
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_free_wire_sync_rust2dart_dco(value: WireSyncRust2DartDco) {
    let _ = Rust2DartMessageDco::from_raw_wire_sync(value);
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_free_wire_sync_rust2dart_sse(value: WireSyncRust2DartSse) {
    let _ = Rust2DartMessageSse::from_raw_wire_sync(value);
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_get_shutdown_callback() -> unsafe extern "C" fn(*mut c_void) {
    /// Called by Dart's `NativeFinalizer` on isolate group shutdown.
    unsafe extern "C" fn frb_shutdown_callback(_: *mut c_void) {
        unsafe extern "C" fn devnull(_: DartPort, _: *mut DartCObject) -> bool {
            // Returning true is wrong since message is not enqueued and this
            // might cause memory leaks. But since application is shutting down
            // we don't really care and just want it to die silently without
            // triggering any send errors.
            true
        }

        // So `Dart_PostCObject` won't do anything from now on. We need this
        // cause once shutdown have started `Dart_Cleanup` might be called any
        // moment from now and `Dart_PostCObject` can only be used before
        // `Dart_Cleanup` has been called
        // For more information refer to:
        // https://github.com/dart-lang/native/issues/2079
        store_dart_post_cobject(devnull);
    }

    frb_shutdown_callback
}
