// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_web_locking.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

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

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_opaque_web_locking_create_twin_sse(
    initial: i32,
) -> RustOpaque<RustOpaqueWebLockingData> {
    RustOpaque::new(RustOpaqueWebLockingData { value: initial })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_opaque_web_locking_get_twin_sse(arg: RustOpaque<RustOpaqueWebLockingData>) -> i32 {
    arg.read().unwrap().value
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn rust_opaque_web_locking_sync_add_twin_sse(
    arg: RustOpaque<RustOpaqueWebLockingData>,
    adder: i32,
) -> i32 {
    let mut arg = arg.write().unwrap();
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_opaque_web_locking_worker_add_twin_sse(
    arg: RustOpaque<RustOpaqueWebLockingData>,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    let mut arg = arg.write().unwrap();
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_opaque_web_locking_async_add_twin_sse(
    arg: RustOpaque<RustOpaqueWebLockingData>,
    adder: i32,
) -> i32 {
    yield_once().await;
    let mut arg = arg.write().unwrap();
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_opaque_web_locking_hold_mut_borrow_forever_twin_sse(
    arg: RustOpaque<RustOpaqueWebLockingData>,
) {
    let _arg = arg.write().unwrap();
    futures::future::pending::<()>().await;
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_web_locking_create_twin_sse(initial: i32) -> RustAutoOpaqueWebLockingData {
    RustAutoOpaqueWebLockingData { value: initial }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_web_locking_get_twin_sse(arg: &RustAutoOpaqueWebLockingData) -> i32 {
    arg.value
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_web_locking_sync_add_twin_sse(
    arg: &mut RustAutoOpaqueWebLockingData,
    adder: i32,
) -> i32 {
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_web_locking_worker_add_twin_sse(
    arg: &mut RustAutoOpaqueWebLockingData,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_web_locking_async_add_twin_sse(
    arg: &mut RustAutoOpaqueWebLockingData,
    adder: i32,
) -> i32 {
    yield_once().await;
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_web_locking_hold_mut_borrow_forever_twin_sse(
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
