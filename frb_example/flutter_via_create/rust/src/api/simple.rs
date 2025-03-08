use crate::frb_generated::StreamSink;
use flutter_rust_bridge::{frb, DartFnFuture};
use std::thread::sleep;
use std::time::Duration;

pub struct Test(Box<dyn Fn() -> DartFnFuture<()> + Send + Sync>);

impl Test {
    #[frb(sync)]
    pub fn new(cb: impl Fn() -> DartFnFuture<()> + Send + Sync + 'static) -> Self {
        Test(Box::new(cb))
    }

    pub async fn call(&self) {
        self.0().await;
    }
}

pub fn stream_fn(s: StreamSink<i32>) {
    loop {
        s.add(42).unwrap();
        sleep(Duration::from_secs(1));
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
