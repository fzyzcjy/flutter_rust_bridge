// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;
use std::sync::Mutex;

static INIT_DART_CODE_MESSAGES: Mutex<Vec<String>> = Mutex::new(vec![]);

#[frb(init_dart_code = "recordInitDartCodeMessage(message: 'first');")]
#[frb(init_dart_code = "recordInitDartCodeMessage(message: 'second');")]
pub fn request_init_dart_code_message() {}

#[frb(sync)]
pub fn record_init_dart_code_message(message: String) {
    let mut messages = INIT_DART_CODE_MESSAGES.lock().unwrap();
    if !messages.contains(&message) {
        messages.push(message);
    }
}

#[frb(sync)]
pub fn get_init_dart_code_messages() -> Vec<String> {
    INIT_DART_CODE_MESSAGES.lock().unwrap().clone()
}
