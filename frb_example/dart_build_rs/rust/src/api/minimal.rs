use flutter_rust_bridge::frb;

flutter_rust_bridge::enable_frb_rust_to_dart_logging!();

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_backtrace();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn emit_log_message(message: String) {
    log::warn!(target: "frb_build_rs_test", "{message}");
}
