use flutter_rust_bridge::frb;
use log::{Level, Log, Metadata, Record, RecordBuilder};

use crate::api::log_2_dart::LOG2DART;

// use log::{logger, LevelFilter, Record};

#[frb(init)]
pub fn init_app() {
    // flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    log::info!(
        "From Rust: Minimal adder called with params {:?} and {:?}",
        a,
        b
    );
    a + b
}
