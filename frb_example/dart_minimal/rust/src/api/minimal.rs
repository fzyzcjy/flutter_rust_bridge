use chrono::{DateTime, Utc};
use flutter_rust_bridge::frb;
use uuid::Uuid;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn gen_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn get_now() -> DateTime<Utc> {
    Utc::now()
}
