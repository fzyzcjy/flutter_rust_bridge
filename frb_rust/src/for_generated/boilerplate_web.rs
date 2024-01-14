#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_boilerplate_web {
    () => {
        $crate::frb_generated_web_cst_codec!();
        $crate::frb_generated_web_extern_func!();
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_web_cst_codec {
    () => {
        impl<T> CstDecode<Option<T>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
        where
            JsValue: CstDecode<T>,
        {
            fn cst_decode(self) -> Option<T> {
                (!self.is_null() && !self.is_undefined()).then(|| self.cst_decode())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_web_extern_func {
    () => {
        #[no_mangle]
        pub extern "C" fn frb_pde_ffi_dispatcher(
            func_id: i32,
            port: $crate::for_generated::MessagePort,
            ptr: $crate::for_generated::PlatformGeneralizedUint8ListPtr,
            rust_vec_len: i32,
            data_len: i32,
        ) {
            pde_ffi_dispatcher(func_id, port, ptr, rust_vec_len, data_len)
        }
    };
}
