use flutter_rust_bridge::frb;
use std::future::poll_fn;
use std::task::Poll;
use std::time::Duration;

#[frb(opaque)]
pub struct RustOpaqueWebLockingData {
    value: i32,
}

pub fn rust_opaque_web_locking_create(initial: i32) -> RustOpaqueWebLockingData {
    RustOpaqueWebLockingData { value: initial }
}

pub fn rust_opaque_web_locking_get(arg: &RustOpaqueWebLockingData) -> i32 {
    arg.value
}

#[frb(sync)]
pub fn rust_opaque_web_locking_sync_add(arg: &mut RustOpaqueWebLockingData, adder: i32) -> i32 {
    arg.value += adder;
    arg.value
}

pub fn rust_opaque_web_locking_worker_add(
    arg: &mut RustOpaqueWebLockingData,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    arg.value += adder;
    arg.value
}

pub async fn rust_opaque_web_locking_async_add(
    arg: &mut RustOpaqueWebLockingData,
    adder: i32,
) -> i32 {
    yield_once().await;
    arg.value += adder;
    arg.value
}

pub async fn rust_opaque_web_locking_hold_mut_borrow_forever(_arg: &mut RustOpaqueWebLockingData) {
    futures::future::pending::<()>().await;
}

async fn yield_once() {
    let mut yielded = false;
    poll_fn(move |cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await
}
