#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_boilerplate_io {
    () => {
        $crate::frb_generated_io_cst_codec!();
        $crate::frb_generated_io_content_hash!();
        $crate::frb_generated_io_extern_func!();
    };
    (
        frb_get_rust_content_hash = $frb_get_rust_content_hash:literal,
        frb_pde_ffi_dispatcher_primary = $frb_pde_ffi_dispatcher_primary:literal,
        frb_pde_ffi_dispatcher_sync = $frb_pde_ffi_dispatcher_sync:literal,
        frb_dart_fn_deliver_output = $frb_dart_fn_deliver_output:literal,
    ) => {
        $crate::frb_generated_io_cst_codec!();
        $crate::frb_generated_io_content_hash!(link_name = $frb_get_rust_content_hash,);
        $crate::frb_generated_io_extern_func!(
            frb_pde_ffi_dispatcher_primary = $frb_pde_ffi_dispatcher_primary,
            frb_pde_ffi_dispatcher_sync = $frb_pde_ffi_dispatcher_sync,
            frb_dart_fn_deliver_output = $frb_dart_fn_deliver_output,
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_io_cst_codec {
    () => {
        pub trait NewWithNullPtr {
            fn new_with_null_ptr() -> Self;
        }

        impl<T> NewWithNullPtr for *mut T {
            fn new_with_null_ptr() -> Self {
                std::ptr::null_mut()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_io_content_hash {
    () => {
        #[no_mangle]
        pub extern "C" fn frb_get_rust_content_hash() -> i32 {
            FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH
        }
    };
    (link_name = $link_name:literal,) => {
        #[unsafe(export_name = $link_name)]
        pub extern "C" fn frb_get_rust_content_hash() -> i32 {
            FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH
        }
    };
}
