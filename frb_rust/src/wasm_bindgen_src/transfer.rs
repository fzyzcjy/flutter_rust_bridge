use crate::ffi::web::*;
use js_sys::{global, Array};
use std::iter::FromIterator;
use web_sys::{DedicatedWorkerGlobalScope, Worker};

impl TransferClosure<JsValue> {
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
