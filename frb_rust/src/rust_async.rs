// TODO move this file etc

use lazy_static::lazy_static;
use parking_lot::Mutex;
use tokio::runtime::Runtime;

lazy_static! {
    static ref ASYNC_RUNTIME: Mutex<Runtime> = Runtime::new().unwrap();
}

pub(crate) fn init() {
    // TODO more customizations
}
