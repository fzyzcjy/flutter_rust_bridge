#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_boilerplate_io {
    () => {
        $crate::frb_generated_io_cst_codec!();
        $crate::frb_generated_io_extern_func!();
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
macro_rules! frb_generated_io_extern_func {
    () => {
        #[no_mangle]
        pub extern "C" fn frb_pde_ffi_dispatcher_primary(
            func_id: i32,
            port: i64,
            ptr: *mut u8,
            rust_vec_len: i32,
            data_len: i32,
        ) {
            pde_ffi_dispatcher_primary_impl(func_id, port, ptr, rust_vec_len, data_len)
        }

        #[no_mangle]
        pub extern "C" fn frb_pde_ffi_dispatcher_sync(
            func_id: i32,
            ptr: *mut u8,
            rust_vec_len: i32,
            data_len: i32,
        ) -> $crate::for_generated::WireSyncRust2DartSse {
            pde_ffi_dispatcher_sync_impl(func_id, ptr, rust_vec_len, data_len)
        }
    };
}
