#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_boilerplate_web {
    () => {
        $crate::frb_generated_web_cst_codec!();
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
