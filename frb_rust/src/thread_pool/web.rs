use crate::web_transfer::transfer_closure::TransferClosure;
use wasm_bindgen::JsValue;

pub use crate::third_party::wasm_bindgen::worker_pool::WorkerPool as ThreadPool;

pub trait BaseThreadPool {
    fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue>;
}
