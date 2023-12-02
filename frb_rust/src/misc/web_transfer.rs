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

type RawClosure<T> = Box<dyn FnOnce(&[T]) + Send + 'static>;

pub struct TransferClosure<T> {
    pub(crate) data: Vec<T>,
    pub(crate) transfer: Vec<T>,
    pub(crate) closure: RawClosure<T>,
}

pub struct TransferClosurePayload<T> {
    pub(crate) func: RawClosure<T>,
}

impl TransferClosure<JsValue> {
    pub fn new(
        data: Vec<JsValue>,
        transfer: Vec<JsValue>,
        closure: impl FnOnce(&[JsValue]) + Send + 'static,
    ) -> Self {
        let closure = Box::new(closure);
        Self {
            data,
            transfer,
            closure,
        }
    }

    // Copied and modified from the wasm_bindgen raytrace-parallel example
    // File: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/raytrace-parallel/src/pool.rs
    /// Posts a <code>[*mut [TransferClosurePayload], ...[JsValue]]</code> message to this worker.
    ///
    /// The worker's `onmessage` should run the corresponding [`receive_transfer_closure`]
    /// to receive the message.
    pub fn apply(self, worker: &Worker) -> Result<(), JsValue> {
        let transfer = self.transfer.into_iter().filter(|value| value.is_truthy());
        let transfer = Array::from_iter(transfer);
        let data = Array::from_iter(self.data);
        // The worker is responsible for cleaning up the leak here.
        let payload = Box::into_raw(Box::new(TransferClosurePayload { func: self.closure }));
        data.unshift(&JsValue::from(payload as i32));
        worker
            .post_message_with_transfer(&data, &transfer)
            .map_err(|err| {
                // post message failed, ownership remains with us.
                drop(unsafe { Box::from_raw(payload) });
                err
            })
    }
}

// Copied and modified from the wasm_bindgen raytrace-parallel example
// File: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/raytrace-parallel/src/pool.rs
/// ## Safety
/// This function reclaims a raw pointer created by [`TransferClosure`], and therefore
/// should **only** be used in conjunction with it.
/// Furthermore, the WASM module in the worker must have been initialized with the shared
/// memory from the host JS scope.
// wasm_bindgen cannot work with unsafe functions, hence the clippy ignore.
#[wasm_bindgen]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn receive_transfer_closure(
    payload: *mut TransferClosurePayload<JsValue>,
    transfer: Box<[JsValue]>,
) -> Result<(), JsValue> {
    let closure = unsafe { Box::from_raw(payload) };
    (closure.func)(&transfer);
    // Once we're done here, notify the host scope so that it can reclaim this worker.
    global()
        .unchecked_into::<DedicatedWorkerGlobalScope>()
        .post_message(&JsValue::UNDEFINED)
}
