use crate::web_transfer::transfer_closure::TransferClosure;
use futures::SinkExt;
use std::panic::RefUnwindSafe;
use std::thread::LocalKey;
use wasm_bindgen::JsValue;

pub use crate::third_party::wasm_bindgen::worker_pool::WorkerPool as SimpleThreadPool;

pub trait BaseThreadPool: RefUnwindSafe {
    fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue>;
}

impl BaseThreadPool for &'static LocalKey<SimpleThreadPool> {
    fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue> {
        self.with(|inner| inner.execute(closure))
    }
}
