use flutter_rust_bridge::frb;

// use log::{logger, LevelFilter, Record};

#[frb(init)]
pub fn init_app() {
    // flutter_rust_bridge::setup_default_user_utils();
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
