use flutter_rust_bridge::{frb, DartFnFuture};

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

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
