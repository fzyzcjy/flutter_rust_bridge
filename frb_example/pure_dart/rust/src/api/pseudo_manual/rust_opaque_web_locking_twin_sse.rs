// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_web_locking.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

use crate::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;
use std::future::poll_fn;
use std::task::Poll;
use std::time::Duration;

#[frb(opaque)]
pub struct RustOpaqueWebLockingDataTwinSse {
    value: i32,
}

pub struct RustAutoOpaqueWebLockingDataTwinSse {
    value: i32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_opaque_web_locking_create_twin_sse(initial: i32) -> RustOpaqueWebLockingDataTwinSse {
    RustOpaqueWebLockingDataTwinSse { value: initial }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_opaque_web_locking_get_twin_sse(arg: &RustOpaqueWebLockingDataTwinSse) -> i32 {
    arg.value
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn rust_opaque_web_locking_sync_add_twin_sse(
    arg: &mut RustOpaqueWebLockingDataTwinSse,
    adder: i32,
) -> i32 {
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_opaque_web_locking_worker_add_twin_sse(
    arg: &mut RustOpaqueWebLockingDataTwinSse,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_opaque_web_locking_async_add_twin_sse(
    arg: &mut RustOpaqueWebLockingDataTwinSse,
    adder: i32,
) -> i32 {
    yield_once().await;
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_opaque_web_locking_hold_mut_borrow_forever_twin_sse(
    _arg: &mut RustOpaqueWebLockingDataTwinSse,
) {
    futures::future::pending::<()>().await;
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_web_locking_create_twin_sse(
    initial: i32,
) -> RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinSse> {
    RustAutoOpaque::new(RustAutoOpaqueWebLockingDataTwinSse { value: initial })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_web_locking_get_twin_sse(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinSse>,
) -> i32 {
    arg.blocking_read().value
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_web_locking_sync_add_twin_sse(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinSse>,
    adder: i32,
) -> i32 {
    let mut arg = arg.blocking_write();
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub fn rust_auto_opaque_web_locking_worker_add_twin_sse(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinSse>,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    let mut arg = arg.blocking_write();
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_web_locking_async_add_twin_sse(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinSse>,
    adder: i32,
) -> i32 {
    yield_once().await;
    let mut arg = arg.write().await;
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_auto_opaque_web_locking_hold_mut_borrow_forever_twin_sse(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinSse>,
) {
    let _arg = arg.write().await;
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
