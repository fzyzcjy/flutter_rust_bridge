use crate::third_party::wasm_bindgen::worker_pool::WorkerPool;

pub struct SimpleThreadPool(WorkerPool);

#[derive(Debug, Clone)]
pub struct SimpleThreadPoolConfig {
    pub max_workers: Option<usize>,
}

impl Default for SimpleThreadPoolConfig {
    fn default() -> Self {
        Self {
            max_workers: None,
        }
    }
}

impl SimpleThreadPool {
    pub fn new_with_config(config: SimpleThreadPoolConfig) -> Result<Self, wasm_bindgen::JsValue> {
        match config.max_workers {
            Some(max_workers) => WorkerPool::new_with_max_workers(max_workers).map(SimpleThreadPool),
            None => Ok(SimpleThreadPool(WorkerPool::default())), // Unbounded
        }
    }

    pub fn new_with_max_workers(max_workers: usize) -> Result<Self, wasm_bindgen::JsValue> {
        Self::new_with_config(SimpleThreadPoolConfig { max_workers: Some(max_workers) })
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
