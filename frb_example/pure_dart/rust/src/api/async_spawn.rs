// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

pub async fn simple_use_async_spawn(arg: String) -> String {
    // ref demo in https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
    let handle = flutter_rust_bridge::spawn(async move { arg.repeat(2) });
    let output = handle.await.unwrap();
    output
}

pub async fn simple_use_async_spawn_blocking(arg: String) -> String {
    // ref demo in https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
    let handle = flutter_rust_bridge::spawn_blocking(move || arg.repeat(2));
    let output = handle.await.unwrap();
    output
}
