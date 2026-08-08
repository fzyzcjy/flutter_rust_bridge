use flutter_rust_bridge::frb;
use std::sync::{Arc, RwLock};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Internal {
    value: String,
}

#[derive(Clone, Debug)]
#[frb(opaque)]
pub struct Handle {
    pub field: Arc<RwLock<Internal>>,
}

pub fn create_handle() -> Handle {
    Handle {
        field: Arc::new(RwLock::new(Internal {
            value: "value".to_owned(),
        })),
    }
}
