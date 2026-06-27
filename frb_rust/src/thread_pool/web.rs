use crate::third_party::wasm_bindgen::worker_pool::WorkerPool;
use crate::web_transfer::transfer_closure::TransferClosure;
use std::thread::LocalKey;
use wasm_bindgen::JsValue;

pub struct SimpleThreadPool(Option<WorkerPool>);

pub trait BaseThreadPool {
    fn execute(&self, closure: TransferClosure<JsValue>);
}

impl BaseThreadPool for &'static LocalKey<SimpleThreadPool> {
    fn execute(&self, closure: TransferClosure<JsValue>) {
        self.with(|inner| inner.execute(closure)).unwrap()
    }
}

impl SimpleThreadPool {
    fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue> {
        match &self.0 {
            Some(inner) => inner.execute(closure),
            None => {
                closure.run_inline();
                Ok(())
            }
        }
    }
}

impl Default for SimpleThreadPool {
    fn default() -> Self {
        match WorkerPool::new(None, None, None, None) {
            Ok(inner) => Self(Some(inner)),
            Err(err) => {
                crate::console_error!("Failed to initialize web worker pool: {:?}", err);
                Self(None)
            }
        }
    }
}
