//! Multiple generated bridges must link without overlapping native exports.
#![cfg(not(target_family = "wasm"))]

macro_rules! bridge {
    ($module:ident, $prefix:literal, $hash:expr) => {
        mod $module {
            use flutter_rust_bridge::for_generated::{Dart2RustMessageSse, WireSyncRust2DartSse};
            const FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH: i32 = $hash;
            flutter_rust_bridge::frb_generated_boilerplate_io!($prefix);
            static FLUTTER_RUST_BRIDGE_HANDLER: Handler = Handler;
            struct Handler;
            impl Handler {
                fn dart_fn_handle_output(&self, id: i32, _: Dart2RustMessageSse) {
                    LAST_CALLBACK.store(id, std::sync::atomic::Ordering::SeqCst);
                }
            }
            pub static LAST_CALLBACK: std::sync::atomic::AtomicI32 =
                std::sync::atomic::AtomicI32::new(0);
            pub static LAST_CALL: std::sync::atomic::AtomicI32 =
                std::sync::atomic::AtomicI32::new(0);
            fn pde_ffi_dispatcher_primary_impl(id: i32, _: i64, _: *mut u8, _: i32, _: i32) {
                LAST_CALL.store(id, std::sync::atomic::Ordering::SeqCst);
            }
            fn pde_ffi_dispatcher_sync_impl(
                _: i32,
                _: *mut u8,
                _: i32,
                _: i32,
            ) -> WireSyncRust2DartSse {
                WireSyncRust2DartSse {
                    ptr: std::ptr::null_mut(),
                    len: $hash,
                }
            }
        }
    };
}

bridge!(package_a, "package_a_", 11);
bridge!(package_b, "package_b_", 22);
bridge!(legacy, "", 33);

extern "C" {
    #[link_name = "package_a_frb_get_rust_content_hash"]
    fn hash_a() -> i32;
    #[link_name = "package_b_frb_get_rust_content_hash"]
    fn hash_b() -> i32;
    #[link_name = "frb_get_rust_content_hash"]
    fn hash_legacy() -> i32;
}

#[test]
fn bridges_export_independent_hashes_in_one_binary() {
    unsafe {
        assert_eq!(hash_a(), 11);
        assert_eq!(hash_b(), 22);
        assert_eq!(hash_legacy(), 33);
    }
}

extern "C" {
    #[link_name = "package_a_frb_pde_ffi_dispatcher_primary"]
    fn primary_a(id: i32, port: i64, ptr: *mut u8, len: i32, data_len: i32);
    #[link_name = "package_b_frb_pde_ffi_dispatcher_primary"]
    fn primary_b(id: i32, port: i64, ptr: *mut u8, len: i32, data_len: i32);
    #[link_name = "package_a_frb_pde_ffi_dispatcher_sync"]
    fn sync_a(
        id: i32,
        ptr: *mut u8,
        len: i32,
        data_len: i32,
    ) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse;
    #[link_name = "package_b_frb_pde_ffi_dispatcher_sync"]
    fn sync_b(
        id: i32,
        ptr: *mut u8,
        len: i32,
        data_len: i32,
    ) -> flutter_rust_bridge::for_generated::WireSyncRust2DartSse;
    #[link_name = "package_a_frb_dart_fn_deliver_output"]
    fn callback_a(id: i32, ptr: *mut u8, len: i32, data_len: i32);
    #[link_name = "package_b_frb_dart_fn_deliver_output"]
    fn callback_b(id: i32, ptr: *mut u8, len: i32, data_len: i32);
}

#[test]
fn dispatchers_route_to_their_own_bridge() {
    unsafe {
        primary_a(101, 0, std::ptr::null_mut(), 0, 0);
        primary_b(202, 0, std::ptr::null_mut(), 0, 0);
        assert_eq!(
            package_a::LAST_CALL.load(std::sync::atomic::Ordering::SeqCst),
            101
        );
        assert_eq!(
            package_b::LAST_CALL.load(std::sync::atomic::Ordering::SeqCst),
            202
        );
        assert_eq!(sync_a(0, std::ptr::null_mut(), 0, 0).len, 11);
        assert_eq!(sync_b(0, std::ptr::null_mut(), 0, 0).len, 22);
    }
    unsafe {
        // Vec::new has a non-null dangling pointer, valid for a zero-length wire buffer.
        let mut first = Vec::<u8>::new();
        let mut second = Vec::<u8>::new();
        let first_ptr = first.as_mut_ptr();
        let second_ptr = second.as_mut_ptr();
        std::mem::forget(first);
        std::mem::forget(second);
        callback_a(303, first_ptr, 0, 0);
        callback_b(404, second_ptr, 0, 0);
    }
    assert_eq!(
        package_a::LAST_CALLBACK.load(std::sync::atomic::Ordering::SeqCst),
        303
    );
    assert_eq!(
        package_b::LAST_CALLBACK.load(std::sync::atomic::Ordering::SeqCst),
        404
    );
}
