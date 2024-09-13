use flutter_rust_bridge::frb;

use crate::log_2_dart::Log2Dart;
use log::LevelFilter;

static LOG2DART: Log2Dart = Log2Dart {};

#[frb(init)]
pub fn init_app() {
    log::set_logger(&LOG2DART)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Could not set logger");

    flutter_rust_bridge::setup_default_user_utils();
    log::info!("Initialized app");
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    log::info!("Minimal adder called with params {:?} and {:?}", a, b);
    a + b
}
