use crate::third_party::wasm_bindgen::worker_pool::WorkerPool;

pub struct SimpleThreadPool(WorkerPool);

impl SimpleThreadPool {
    pub fn new_with_max_workers(max_workers: usize) -> Result<Self, wasm_bindgen::JsValue> {
        WorkerPool::new_with_max_workers(max_workers).map(SimpleThreadPool)
    }
}

impl Default for SimpleThreadPool {
    fn default() -> Self {
        SimpleThreadPool(WorkerPool::default())
    }
}
use crate::web_transfer::transfer_closure::TransferClosure;
use std::thread::LocalKey;
use wasm_bindgen::JsValue;

pub trait BaseThreadPool {
    fn execute(&self, closure: TransferClosure<JsValue>);
}

impl BaseThreadPool for &'static LocalKey<SimpleThreadPool> {
    fn execute(&self, closure: TransferClosure<JsValue>) {
        self.with(|inner| inner.0.execute(closure)).unwrap()
    }
}

impl BaseThreadPool for SimpleThreadPool {
    fn execute(&self, closure: TransferClosure<JsValue>) {
        self.0.execute(closure).unwrap()
    }
}
