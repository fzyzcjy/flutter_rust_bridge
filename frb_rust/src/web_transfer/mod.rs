#[cfg(target_family = "wasm")]
pub(crate) mod transfer;
#[cfg(target_family = "wasm")]
pub(crate) mod transfer_closure;

/// On WASM, `JsValue`s cannot be shared between scopes but instead can be
/// ["transferred"]. Rust however is not aware of transferables and therefore cannot
/// capture these values. This macro wraps a closure and returns a `TransferClosure` on WASM platforms
/// which will capture these special values, or a normal [FnOnce] on other platforms.
/// Note that the parameter names must match available variables/bindings from the outer scope.
///
/// ["transferred"]: https://developer.mozilla.org/en-US/docs/Glossary/Transferable_objects
#[macro_export]
macro_rules! transfer {
    (|| $block:block) => {{
        #[cfg(not(target_family = "wasm"))]
        { move || $block }

        #[cfg(target_family = "wasm")]
        {
            $crate::for_generated::TransferClosure::new(vec![], vec![], move |_: &[wasm_bindgen::JsValue]| $block)
        }
    }};
    (|$($param:ident: $ty:ty),*| $block:block) => {{
        #[cfg(not(target_family = "wasm"))]
        {
            move || $block
        }

        #[cfg(target_family = "wasm")]
        {
            use $crate::web_transfer::transfer::Transfer;
            #[allow(unused_variables)]
            let worker = move |transfer: &[wasm_bindgen::JsValue]| {
                let idx = 0;
                $(
                    let $param = <$ty>::deserialize(&transfer[idx]);
                    let idx = idx + 1;
                )*
                $block
            };
            let transferables = [$($param.transferables()),*].concat();
            $crate::for_generated::TransferClosure::new(vec![$($param.serialize()),*], transferables, worker)
        }
    }};
}
