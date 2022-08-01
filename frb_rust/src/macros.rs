/// On WASM, [JsValue][wasm_bindgen::JsValue]s cannot be shared between workers but instead can be
/// "transferred". Rust however is not aware of transferables and therefore cannot
/// capture these values. This macro returns a [TransferClosure] on WASM platforms
/// which will capture these special values, and a normal [FnOnce] on other platforms.
/// Note that the parameter names must match available variables/bindings from the outer scope.
#[macro_export]
macro_rules! transfer {
    (|$($param:ident: $ty:ty),*| { $($body:tt)* }) => {{
        #[cfg(not(target_family = "wasm"))]
        {
            move || { $($body)* }
        }

        #[cfg(target_family = "wasm")]
        {
            use wasm_bindgen::JsValue;
            use $crate::ffi::FromDart;
            #[allow(unused_assignments)]
            let worker = move |transfer: &[JsValue]| {
                let mut idx = 0;
                $(
                    let $param = <$ty>::from_dart(&transfer[idx]);
                    idx += 1;
                )*
                { $($body)* }
            };
            $crate::ffi::TransferClosure::new(vec![$(JsValue::from($param)),*], worker)
        }
    }};
}

#[macro_export]
macro_rules! console_log {
    ($lit:literal) => {
        #[allow(unused_unsafe)]
        unsafe { $crate::ffi::log($lit) }
    };
    ($( $tt:tt )*) => {
        #[allow(unused_unsafe)]
        unsafe { $crate::ffi::log(&format!($( $tt )*)) }
    };
}

#[macro_export]
macro_rules! console_error {
    ($lit:literal) => {
        #[allow(unused_unsafe)]
        unsafe { $crate::ffi::error($lit) }
    };
    ($( $tt:tt )*) => {
        #[allow(unused_unsafe)]
        unsafe { $crate::ffi::error(&format!($( $tt )*)) }
    };
}
