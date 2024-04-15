#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_boilerplate_web {
    () => {
        $crate::frb_generated_web_cst_codec!();
        $crate::frb_generated_web_content_hash!();
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
macro_rules! frb_generated_web_content_hash {
    () => {
        #[wasm_bindgen]
        pub extern "C" fn frb_get_rust_content_hash() -> i32 {
            FLUTTER_RUST_BRIDGE_CODEGEN_CONTENT_HASH
        }
    };
}
