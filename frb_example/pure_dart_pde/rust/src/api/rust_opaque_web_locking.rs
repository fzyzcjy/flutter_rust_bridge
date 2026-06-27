// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

use crate::frb_generated::RustOpaque;
use flutter_rust_bridge::frb;
use std::future::poll_fn;
use std::task::Poll;
use std::time::Duration;

pub struct RustOpaqueWebLockingData {
    value: i32,
}

#[frb(opaque)]
pub struct RustAutoOpaqueWebLockingData {
    value: i32,
}

pub fn rust_opaque_web_locking_create(initial: i32) -> RustOpaque<RustOpaqueWebLockingData> {
    RustOpaque::new(RustOpaqueWebLockingData { value: initial })
}

pub fn rust_opaque_web_locking_get(arg: RustOpaque<RustOpaqueWebLockingData>) -> i32 {
    arg.read().unwrap().value
}

#[frb(sync)]
pub fn rust_opaque_web_locking_sync_add(
    arg: RustOpaque<RustOpaqueWebLockingData>,
    adder: i32,
) -> i32 {
    let mut arg = arg.write().unwrap();
    arg.value += adder;
    arg.value
}

pub fn rust_opaque_web_locking_worker_add(
    arg: RustOpaque<RustOpaqueWebLockingData>,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    let mut arg = arg.write().unwrap();
    arg.value += adder;
    arg.value
}

pub async fn rust_opaque_web_locking_async_add(
    arg: RustOpaque<RustOpaqueWebLockingData>,
    adder: i32,
) -> i32 {
    yield_once().await;
    let mut arg = arg.write().unwrap();
    arg.value += adder;
    arg.value
}

pub async fn rust_opaque_web_locking_hold_mut_borrow_forever(
    arg: RustOpaque<RustOpaqueWebLockingData>,
) {
    let _arg = arg.write().unwrap();
    futures::future::pending::<()>().await;
}

pub fn rust_auto_opaque_web_locking_create(initial: i32) -> RustAutoOpaqueWebLockingData {
    RustAutoOpaqueWebLockingData { value: initial }
}

pub fn rust_auto_opaque_web_locking_get(arg: &RustAutoOpaqueWebLockingData) -> i32 {
    arg.value
}

#[frb(sync)]
pub fn rust_auto_opaque_web_locking_sync_add(
    arg: &mut RustAutoOpaqueWebLockingData,
    adder: i32,
) -> i32 {
    arg.value += adder;
    arg.value
}

pub fn rust_auto_opaque_web_locking_worker_add(
    arg: &mut RustAutoOpaqueWebLockingData,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    arg.value += adder;
    arg.value
}

pub async fn rust_auto_opaque_web_locking_async_add(
    arg: &mut RustAutoOpaqueWebLockingData,
    adder: i32,
) -> i32 {
    yield_once().await;
    arg.value += adder;
    arg.value
}

pub async fn rust_auto_opaque_web_locking_hold_mut_borrow_forever(
    _arg: &mut RustAutoOpaqueWebLockingData,
) {
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
