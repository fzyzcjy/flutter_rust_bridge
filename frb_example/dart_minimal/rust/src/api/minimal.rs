use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::futures::FutureExt;
use flutter_rust_bridge::frb;
use std::sync::Arc;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn my_async_rust_function() {
    flutter_rust_bridge::console_error!("my_async_rust_function start");

    // let mutex = Arc::new(tokio::sync::Mutex::new(42));

    // https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let num_threads = 10;
    // let num_threads = 3;
    // let num_threads = 100;

    let mut handles = vec![];
    for spawn_id in 0..num_threads {
        let tx2 = tx.clone();
        // let mutex2 = mutex.clone();
        let handle = flutter_rust_bridge::spawn_blocking_with(
            move || {
                let thread_id = std::thread::current().id();

                // for i in 0..100000 {
                //     *mutex2.blocking_lock() += 1;
                //     if i % 1000 == 0 {
                //         flutter_rust_bridge::console_error!(
                //             "thread={:?} mutex={}",
                //             thread_id,
                //             *mutex2.blocking_lock()
                //         );
                //     }
                // }

                for i in 0..10000000 {
                    // let msg = format!("{spawn_id}_{thread_id:?}_{i}");
                    if let Err(_) = tx2.send(1) {
                        flutter_rust_bridge::console_error!("receiver dropped");
                        return;
                    }
                }
            },
            FLUTTER_RUST_BRIDGE_HANDLER.thread_pool(),
        );
        handles.push(handle);
    }
    drop(tx);

    // for i in 0..100000 {
    //     *mutex.lock().await += 1;
    //     if i % 1000 == 0 {
    //         flutter_rust_bridge::console_error!("main thread mutex={}", *mutex.lock().await);
    //     }
    // }

    let mut sum = 0;
    while let Some(i) = rx.recv().await {
        sum += i;
        // flutter_rust_bridge::console_error!("recv: {}", i);
        if sum % 1000 == 0 {
            flutter_rust_bridge::console_error!("main thread sum={}", sum);
        }
    }

    for handle in handles {
        handle.await.unwrap();
    }

    flutter_rust_bridge::console_error!("my_async_rust_function end");
}
