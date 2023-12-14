// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

pub async fn simple_use_async_spawn() {
    // ref demo in https://docs.rs/tokio/latest/tokio/task/fn.spawn.html
    let handle = flutter_rust_bridge::spawn(async { "hello".to_owned() });
    let output = handle.await.unwrap();
    assert_eq!(output, "hello".to_owned());
}

pub async fn simple_use_async_spawn_blocking() {
    // ref demo in https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
    let handle = flutter_rust_bridge::spawn_blocking(|| "hello".to_owned());
    let output = handle.await.unwrap();
    assert_eq!(output, "hello".to_owned());
}
