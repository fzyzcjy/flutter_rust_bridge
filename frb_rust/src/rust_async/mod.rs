// TODO move this file etc
// TODO split to io/web

use lazy_static::lazy_static;
use parking_lot::Mutex;
use tokio::runtime::Runtime;

lazy_static! {
    // TODO do not be public (but encapsulate)
    pub static ref ASYNC_RUNTIME: Mutex<Runtime> = Mutex::new(create_runtime());
}

pub(crate) fn init() {
    // TODO more customizations
}

#[cfg(not(wasm))]
fn create_runtime() -> Runtime {
    Runtime::new().unwrap()
}

#[cfg(wasm)]
fn create_runtime() -> Runtime {
    // TODO create a new thread for this?
    (tokio::runtime::Builder::new_current_thread().build()).unwrap()
}
