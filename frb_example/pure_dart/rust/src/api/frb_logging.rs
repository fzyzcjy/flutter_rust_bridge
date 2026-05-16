// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

flutter_rust_bridge::enable_frb_rust_to_dart_logging!(
    max_level = log::LevelFilter::Warn,
    setup_dart_logging_output = false
);

pub fn emit_log_message() {
    log::warn!("hello from rust logging bridge");
}

pub fn print_to_console_smoke_test() {
    flutter_rust_bridge::for_generated::print_to_console(
        "hello from flutter_rust_bridge print_to_console",
    );
}
