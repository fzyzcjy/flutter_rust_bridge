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
