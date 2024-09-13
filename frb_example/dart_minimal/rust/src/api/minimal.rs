use flutter_rust_bridge::frb;

use crate::log_2_dart::Log2Dart;
use log::LevelFilter;

use std::sync::OnceLock;

static LOG2DART: OnceLock<Log2Dart> = OnceLock::new();

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();

    let log_fn = |msg| {
        println!("callback: {}", msg);
        msg
    };
    LOG2DART.set(Log2Dart::new(
        // |msg| println!("callback: {}", msg),
        log_fn,
    ));
    // TODO ignore error, as FRB generates the same call, so it get's called twoce. Should be fixed once this is part of the generated functions
    let _ = log::set_logger(LOG2DART.get().expect("log2dart has been instantiated"))
        .map(|()| log::set_max_level(LevelFilter::Info));
    // .expect("Could not set logger");
    log::info!("From Rust: Initialized app");
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    log::info!(
        "From Rust: Minimal adder called with params {:?} and {:?}",
        a,
        b
    );
    a + b
}
