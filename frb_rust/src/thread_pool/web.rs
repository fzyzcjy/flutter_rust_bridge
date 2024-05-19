pub use crate::third_party::wasm_bindgen::worker_pool::WorkerPool as SimpleThreadPool;
use crate::web_transfer::transfer_closure::TransferClosure;
use std::thread::LocalKey;
use wasm_bindgen::JsValue;
use crate::console_error;

pub trait BaseThreadPool {
    fn execute(&self, closure: TransferClosure<JsValue>);
}

impl BaseThreadPool for &'static LocalKey<SimpleThreadPool> {
    fn execute(&self, closure: TransferClosure<JsValue>) {
        console_error!("hi SimpleThreadPool.execute START");
        let result = self.try_with(|inner| {
            console_error!("hi SimpleThreadPool.execute inside with");
            inner.execute(closure)
        });
        console_error!("hi SimpleThreadPool.execute 2 result.is_ok={:?}", result.is_ok());
        todo!()
    }
}
