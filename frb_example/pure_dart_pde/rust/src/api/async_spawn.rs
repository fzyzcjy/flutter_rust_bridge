// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use crate::frb_generated::{StreamSink, FLUTTER_RUST_BRIDGE_HANDLER};

pub async fn simple_use_async_spawn(arg: String) -> String {
    // ref demo in https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
    let handle = flutter_rust_bridge::spawn(async move { arg.repeat(2) });
    handle.await.unwrap()
}

pub async fn simple_use_async_spawn_blocking(arg: String) -> String {
    // ref demo in https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
    let handle = flutter_rust_bridge::spawn_blocking_with(
        move || arg.repeat(2),
        FLUTTER_RUST_BRIDGE_HANDLER.thread_pool(),
    );
    handle.await.unwrap()
}

// NOTE: Often the `spawn` and `spawn_blocking` is enough
pub async fn simple_use_async_spawn_local(arg: String, sink: StreamSink<String>) {
    let core = || async move {
        let handle = flutter_rust_bridge::spawn_local(async move { arg.repeat(2) });
        sink.add(handle.await.unwrap()).unwrap();
    };

    // Usually you will not use like this. This is just a test to ensure the function works.
    // ref tokio demo in https://docs.rs/tokio/latest/tokio/task/fn.spawn_local.html
    // and how tokio `LocalSet` is used https://docs.rs/tokio/latest/tokio/task/struct.LocalSet.html#use-inside-tokiospawn
    #[cfg(not(target_family = "wasm"))]
    {
        use tokio::runtime::Builder;
        use tokio::task::LocalSet;

        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        std::thread::spawn(move || {
            let local = LocalSet::new();
            local.spawn_local(async move { core().await });
            rt.block_on(local);
        });
    }

    #[cfg(target_family = "wasm")]
    core().await;
}
