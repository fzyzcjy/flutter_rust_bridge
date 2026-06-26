use crate::codec::dco::Rust2DartMessageDco;
use crate::codec::sse::Rust2DartMessageSse;
use crate::codec::Rust2DartMessageTrait;
use crate::misc::atomic::AtomicU64;
use crate::platform_types::{WireSyncRust2DartDco, WireSyncRust2DartSse};
use allo_isolate::{
    ffi::{DartCObject, DartPort},
    store_dart_post_cobject,
};
use std::ffi::c_void;
#[cfg(frb_sanitize_memory)]
use std::ffi::{c_char, c_int};
#[cfg(frb_sanitize_memory)]
use std::mem::size_of;
use std::sync::atomic::Ordering;

#[cfg(frb_sanitize_memory)]
const DART_API_DL_MAX_ENTRIES: usize = 512;
#[cfg(frb_sanitize_memory)]
const DART_API_DL_MAX_NAME_BYTES: usize = 128;

#[cfg(frb_sanitize_memory)]
#[repr(C)]
struct DartApiDlEntry {
    name: *const c_char,
    function: *const c_void,
}

#[cfg(frb_sanitize_memory)]
#[repr(C)]
struct DartApiDlData {
    major: c_int,
    minor: c_int,
    functions: *const DartApiDlEntry,
}

#[cfg(frb_sanitize_memory)]
extern "C" {
    fn __msan_unpoison(addr: *const c_void, size: usize);
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_init_frb_dart_api_dl(data: *mut std::ffi::c_void) -> isize {
    // This is compiled only for MSAN-specific sanitizer builds; regular coverage does not see that cfg.
    // frb-coverage:ignore-start
    #[cfg(frb_sanitize_memory)]
    unpoison_dart_api_dl_data_for_msan(data);
    // frb-coverage:ignore-end

    #[cfg(feature = "dart-opaque")]
    return dart_sys::Dart_InitializeApiDL(data);
    #[cfg(not(feature = "dart-opaque"))]
    return 0;
}

#[cfg(frb_sanitize_memory)]
unsafe fn unpoison_dart_api_dl_data_for_msan(data: *mut c_void) {
    if data.is_null() {
        return;
    }

    __msan_unpoison(data, size_of::<DartApiDlData>());
    let data = data as *const DartApiDlData;
    let functions = (*data).functions;
    if functions.is_null() {
        return;
    }

    for index in 0..DART_API_DL_MAX_ENTRIES {
        let entry = functions.add(index);
        __msan_unpoison(entry as *const c_void, size_of::<DartApiDlEntry>());
        let name = (*entry).name;
        if name.is_null() {
            break;
        }
        __msan_unpoison(name as *const c_void, DART_API_DL_MAX_NAME_BYTES);
    }
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
pub unsafe extern "C" fn frb_create_shutdown_callback() -> unsafe extern "C" fn(*mut c_void) {
    /// Counter for how many active shutdown callbacks there are.
    ///
    /// It is incremented when `frb_shutdown_callback` is returned and
    /// decremented when it is called.
    static ISOLATES_NUM: AtomicU64 = AtomicU64::new(0);

    _ = ISOLATES_NUM.fetch_add(1, Ordering::SeqCst);

    /// Called by Dart's `NativeFinalizer` on isolate group shutdown.
    unsafe extern "C" fn frb_shutdown_callback(_: *mut c_void) {
        unsafe extern "C" fn devnull(_: DartPort, _: *mut DartCObject) -> bool {
            // Returning true is wrong since message is not enqueued and this
            // might cause memory leaks. But since application is shutting down
            // we don't really care and just want it to die silently without
            // triggering any send errors.
            true
        }

        let running = ISOLATES_NUM.fetch_sub(1, Ordering::SeqCst);

        // If this is the last callback we assume that application is shutting
        // down.
        if running == 1 {
            // So `Dart_PostCObject` won't do anything from now on. We need this
            // cause once shutdown have started `Dart_Cleanup` might be called any
            // moment from now and `Dart_PostCObject` can only be used before
            // `Dart_Cleanup` has been called
            // For more information refer to:
            // https://github.com/dart-lang/native/issues/2079
            store_dart_post_cobject(devnull);
        };
    }

    frb_shutdown_callback
}
