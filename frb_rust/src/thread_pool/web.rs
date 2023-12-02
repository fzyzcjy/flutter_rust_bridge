use crate::for_generated::wasm_bindgen::JsValue;
use crate::web_transfer::transfer_closure::TransferClosure;

pub use crate::third_party::wasm_bindgen::worker_pool::WorkerPool as ThreadPool;

// Note: Can use `thread local` to wrap the thread pool in our Executor,
// because our Executor only access the thread pool from one thread - the Dart main thread.
// But for simplicity, here we use Mutex before this is measured to be a bottleneck.
#[derive(Default)]
pub struct ThreadPoolWrapped(std::sync::Mutex<ThreadPool>);

impl ThreadPoolWrapped {
    pub fn execute(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue> {
        self.0.lock().unwrap().execute(closure)
    }
}
