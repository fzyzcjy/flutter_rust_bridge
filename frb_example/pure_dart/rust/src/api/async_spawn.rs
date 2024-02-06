// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;

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

pub async fn simple_use_async_spawn_local(arg: String) -> String {
    async fn core(arg: String) -> String {
        let handle = flutter_rust_bridge::spawn_local(async move { arg.repeat(2) });
        handle.await.unwrap()
    }

    // ref demo in https://docs.rs/tokio/latest/tokio/task/fn.spawn_local.html
    #[cfg(not(wasm))]
    return tokio::task::LocalSet::new()
        .run_until(async move { core(arg) })
        .await;

    #[cfg(wasm)]
    return core(arg).await;
}
