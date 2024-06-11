use flutter_rust_bridge::frb;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn my_async_rust_function() {
    flutter_rust_bridge::console_error!("my_async_rust_function start");

    // https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut handles = vec![];
    for spawn_id in 0..10 {
        let tx2 = tx.clone();
        let handle = flutter_rust_bridge::spawn_blocking_with(move || {
            let thread_id = std::thread::current().id();
            for i in 0..10 {
                if let Err(_) = tx2.send(format!("{spawn_id}_{thread_id:?}_{i}")) {
                    flutter_rust_bridge::console_error!("receiver dropped");
                    return;
                }
            }
        }, FLUTTER_RUST_BRIDGE_HANDLER.thread_pool());
        handles.push(handle);
    }
    drop(tx);

    while let Some(i) = rx.recv().await {
        flutter_rust_bridge::console_error!("recv: {}", i);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    flutter_rust_bridge::console_error!("my_async_rust_function end");
}
