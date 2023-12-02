pub(crate) mod transfer_closure;
pub(crate) mod transfer;

use crate::ffi::web::*;
use js_sys::{global, Array};
use std::iter::FromIterator;
use web_sys::{DedicatedWorkerGlobalScope, Worker};

/// On WASM, [JsValue][wasm_bindgen::JsValue]s cannot be shared between scopes but instead can be
/// ["transferred"]. Rust however is not aware of transferables and therefore cannot
/// capture these values. This macro wraps a closure and returns a [TransferClosure][crate::ffi::TransferClosure] on WASM platforms
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
            $crate::ffi::TransferClosure::new(vec![], vec![], move |_: &[wasm_bindgen::JsValue]| $block)
        }
    }};
    (|$($param:ident: $ty:ty),*| $block:block) => {{
        #[cfg(not(target_family = "wasm"))]
        {
            move || $block
        }

        #[cfg(target_family = "wasm")]
        {
            use $crate::ffi::Transfer;
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
            $crate::ffi::TransferClosure::new(vec![$($param.serialize()),*], transferables, worker)
        }
    }};
}
