use crate::web_transfer::transfer_closure::TransferClosure;
use futures::SinkExt;
use std::thread::LocalKey;
use wasm_bindgen::JsValue;

pub use crate::third_party::wasm_bindgen::worker_pool::WorkerPool as ThreadPool;

pub trait BaseThreadPool {
    fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue>;
}

impl BaseThreadPool for ThreadPool {
    fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue> {
        self.execute(closure)
    }
}

impl BaseThreadPool for &'static LocalKey<ThreadPool> {
    fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue> {
        self.with(|inner| inner.execute(closure))
    }
}
