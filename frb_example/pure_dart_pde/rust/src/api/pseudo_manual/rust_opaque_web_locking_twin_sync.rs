// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_web_locking.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;
use std::future::poll_fn;
use std::task::Poll;
use std::time::Duration;

#[frb(opaque)]
pub struct RustOpaqueWebLockingData {
    value: i32,
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_opaque_web_locking_create_twin_sync(initial: i32) -> RustOpaqueWebLockingData {
    RustOpaqueWebLockingData { value: initial }
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_opaque_web_locking_get_twin_sync(arg: &RustOpaqueWebLockingData) -> i32 {
    arg.value
}

#[frb(sync)]
#[flutter_rust_bridge::frb(sync)]
pub fn rust_opaque_web_locking_sync_add_twin_sync(
    arg: &mut RustOpaqueWebLockingData,
    adder: i32,
) -> i32 {
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(sync)]
pub fn rust_opaque_web_locking_worker_add_twin_sync(
    arg: &mut RustOpaqueWebLockingData,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(sync)]
pub async fn rust_opaque_web_locking_async_add_twin_sync(
    arg: &mut RustOpaqueWebLockingData,
    adder: i32,
) -> i32 {
    yield_once().await;
    arg.value += adder;
    arg.value
}

#[flutter_rust_bridge::frb(sync)]
pub async fn rust_opaque_web_locking_hold_mut_borrow_forever_twin_sync(
    _arg: &mut RustOpaqueWebLockingData,
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
