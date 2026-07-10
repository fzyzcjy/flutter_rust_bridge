use std::rc::Rc;

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

pub async fn func_async_void_twin_normal() {}

pub async fn func_async_simple_add_twin_normal(a: i32, b: i32) -> i32 {
    a + b
}

#[flutter_rust_bridge::frb(local)]
pub async fn func_async_local_non_send_twin_normal() -> i32 {
    let value = Rc::new(41);
    let local_task = flutter_rust_bridge::spawn_local(async { 1 });
    tokio::task::yield_now().await;
    *value + local_task.await.unwrap()
}
