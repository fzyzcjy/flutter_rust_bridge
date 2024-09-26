use flutter_rust_bridge::enable_frb_logging;
use flutter_rust_bridge::frb;
use log;

enable_frb_logging!();

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    log::info!("adding {} and {}", a, b);
    a + b
}
