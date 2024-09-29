use flutter_rust_bridge::*;

enable_frb_logging!();
// TODO remove once unit tests are written
// enable_frb_logging!(
//     name = "FRBLogger",
//     maxLoglevel = "INFO",
//     customLogFunction = (|record: Log2DartLogRecord| {
//         let timestamp = chrono::Local::now();
//         let max_log_level = from_u16(record.level_number);
//         let lang = if record.rust_log { "Rust" } else { "Dart" };
//         let logger_name = record.logger_name;
//         let message = record.message;
//         println!("[{timestamp:?} {max_log_level} @{lang} {logger_name}] {message})");
//     })
// );

// enable_frb_logging!(name = "ParamAloneLogger");
// enable_frb_logging!(name = "ParamInfoLogger", maxLoglevel = "warning");
// enable_frb_logging!(maxLoglevel = "warning");
// enable_frb_logging!(
//     customLogFunction = (|record: Log2DartLogRecord| {
//         let logger_name = record.logger_name;
//         let message = record.message;
//         println!("[{logger_name}] {message})");
//     })
// );

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    log::info!("adding {} and {}", a, b);
    a + b
}
