// TODO move this file etc

use tokio::runtime::Runtime;

lazy_static! {
    static ref ASYNC_RUNTIME: Mutex<Runtime> = Runtime::new().unwrap();
}

pub(crate) fn init() {
    todo!()
}
