use crate::frb_generated::StreamSink;
use flutter_rust_bridge::{frb, DartFnFuture, DartOpaque};
use std::thread;
#[cfg(target_family = "wasm")]
use wasm_bindgen_futures::spawn_local as spawn;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(not(target_family = "wasm"))]
fn spawn<F>(fut: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    use std::{
        future,
        sync::{Arc, OnceLock},
    };
    use tokio::runtime::Runtime;

    static RT: OnceLock<Arc<Runtime>> = OnceLock::new();

    RT.get_or_init(|| {
        let rt = Arc::new(Runtime::new().unwrap());

        std::thread::spawn({
            let rt = Arc::clone(&rt);
            move || {
                rt.block_on(async {
                    future::pending::<()>().await;
                });
            }
        });

        rt
    })
    .spawn(fut);
}

pub fn simple_thread_pool_fn(a: i32) -> i32 {
    a
}

#[frb(sync)]
pub fn simple_sync_fn(a: i32) -> i32 {
    a
}

pub fn dart_callback_thread_pool_fn(
    cb: impl Fn() -> DartFnFuture<String>,
    success: impl Fn() -> DartFnFuture<()>,
) {
    let cb_fut = cb();
    let success_fut = success();

    spawn(async move {
        assert_eq!(cb_fut.await, "dart_callback_thread_pool_fn");
        success_fut.await;
    })
}

#[frb(sync)]
pub fn dart_callback_sync_fn(
    cb: impl Fn() -> DartFnFuture<String> + 'static,
    success: impl Fn() -> DartFnFuture<()> + 'static,
) {
    let cb_fut = cb();
    let success_fut = success();

    spawn(async move {
        assert_eq!(cb_fut.await, "dart_callback_sync_fn");
        success_fut.await;
    })
}

pub fn dart_opaque_thread_pool_fn(opaque: DartOpaque) -> DartOpaque {
    opaque
}

#[frb(sync)]
pub fn dart_opaque_sync_fn(opaque: DartOpaque) -> DartOpaque {
    opaque
}

pub fn stream_sink_thread_pool_fn(sink: StreamSink<i32>) {
    thread::spawn(move || {
        sink.add(1).unwrap();
        sink.add(2).unwrap();
        sink.add(3).unwrap();
    });
}

#[frb(sync)]
pub fn stream_sink_sync_fn(sink: StreamSink<i32>) {
    spawn(async move {
        sink.add(1).unwrap();
        sink.add(2).unwrap();
        sink.add(3).unwrap();
    });
}
