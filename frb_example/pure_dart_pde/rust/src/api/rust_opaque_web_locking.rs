// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

use crate::frb_generated::RustAutoOpaque;
use flutter_rust_bridge::frb;
use std::future::poll_fn;
use std::task::Poll;
use std::time::Duration;

#[frb(opaque)]
pub struct RustOpaqueWebLockingDataTwinNormal {
    value: i32,
}

pub struct RustAutoOpaqueWebLockingDataTwinNormal {
    value: i32,
}

pub fn rust_opaque_web_locking_create_twin_normal(
    initial: i32,
) -> RustOpaqueWebLockingDataTwinNormal {
    RustOpaqueWebLockingDataTwinNormal { value: initial }
}

pub fn rust_opaque_web_locking_get_twin_normal(arg: &RustOpaqueWebLockingDataTwinNormal) -> i32 {
    arg.value
}

#[frb(sync)]
pub fn rust_opaque_web_locking_sync_add_twin_normal(
    arg: &mut RustOpaqueWebLockingDataTwinNormal,
    adder: i32,
) -> i32 {
    arg.value += adder;
    arg.value
}

pub fn rust_opaque_web_locking_worker_add_twin_normal(
    arg: &mut RustOpaqueWebLockingDataTwinNormal,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    arg.value += adder;
    arg.value
}

pub async fn rust_opaque_web_locking_async_add_twin_normal(
    arg: &mut RustOpaqueWebLockingDataTwinNormal,
    adder: i32,
) -> i32 {
    yield_once().await;
    arg.value += adder;
    arg.value
}

pub async fn rust_opaque_web_locking_hold_mut_borrow_forever_twin_normal(
    _arg: &mut RustOpaqueWebLockingDataTwinNormal,
) {
    futures::future::pending::<()>().await;
}

pub fn rust_auto_opaque_web_locking_create_twin_normal(
    initial: i32,
) -> RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinNormal> {
    RustAutoOpaque::new(RustAutoOpaqueWebLockingDataTwinNormal { value: initial })
}

pub fn rust_auto_opaque_web_locking_get_twin_normal(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinNormal>,
) -> i32 {
    arg.blocking_read().value
}

#[frb(sync)]
pub fn rust_auto_opaque_web_locking_sync_add_twin_normal(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinNormal>,
    adder: i32,
) -> i32 {
    let mut arg = arg.blocking_write();
    arg.value += adder;
    arg.value
}

pub fn rust_auto_opaque_web_locking_worker_add_twin_normal(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinNormal>,
    adder: i32,
    delay_millis: u32,
) -> i32 {
    std::thread::sleep(Duration::from_millis(delay_millis.into()));
    let mut arg = arg.blocking_write();
    arg.value += adder;
    arg.value
}

pub async fn rust_auto_opaque_web_locking_async_add_twin_normal(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinNormal>,
    adder: i32,
) -> i32 {
    yield_once().await;
    let mut arg = arg.write().await;
    arg.value += adder;
    arg.value
}

pub async fn rust_auto_opaque_web_locking_hold_mut_borrow_forever_twin_normal(
    arg: RustAutoOpaque<RustAutoOpaqueWebLockingDataTwinNormal>,
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
