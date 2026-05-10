// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `attribute.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;
use log::info;
use std::sync::Mutex;

static INIT_DART_CODE_MESSAGES_TWIN_NORMAL: Mutex<Vec<String>> = Mutex::new(vec![]);

#[frb]
#[derive(Debug, Clone)]
pub struct CustomizedTwinRustAsyncSse {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

#[frb(ignore)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn func_should_not_exist_in_dart_twin_rust_async_sse() {}

pub struct StructWithOnlyIgnoredMethodTwinRustAsyncSse {}

impl StructWithOnlyIgnoredMethodTwinRustAsyncSse {
    #[frb(ignore)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn method_should_not_exist_in_dart_twin_rust_async_sse(&self) {}
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_customized_struct_twin_rust_async_sse(val: CustomizedTwinRustAsyncSse) {
    info!("{:#?}", val);
}

#[frb(
    init_dart_code = "api.crateApiAttributeRecordInitDartCodeMessageTwinRustAsyncSse(message: 'first');"
)]
#[frb(
    init_dart_code = "api.crateApiAttributeRecordInitDartCodeMessageTwinRustAsyncSse(message: 'second');"
)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn request_init_dart_code_message_twin_rust_async_sse() {}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn record_init_dart_code_message_twin_rust_async_sse(message: String) {
    let mut messages = INIT_DART_CODE_MESSAGES_TWIN_NORMAL.lock().unwrap();
    if !messages.contains(&message) {
        messages.push(message);
    }
}

#[frb(sync)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn get_init_dart_code_messages_twin_rust_async_sse() -> Vec<String> {
    INIT_DART_CODE_MESSAGES_TWIN_NORMAL.lock().unwrap().clone()
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserIdTwinRustAsyncSse {
    #[frb(default = 0)]
    pub value: u32,
}

#[frb]
#[flutter_rust_bridge::frb(serialize)]
pub async fn next_user_id_twin_rust_async_sse(
    #[frb(default = "const UserIdTwinRustAsyncSse()")] user_id: UserIdTwinRustAsyncSse,
) -> UserIdTwinRustAsyncSse {
    UserIdTwinRustAsyncSse {
        value: user_id.value + 1,
    }
}

// Note: Some attributes are put on `KitchenSinkTwinRustAsyncSse` currently
// (but we can add more tests in this file later)

#[frb(ignore)]
pub struct IgnoredStructTwinRustAsyncSse {
    pub value: u32,
}

impl IgnoredStructTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn method_should_not_exist_in_dart_twin_rust_async_sse(&self) {}
}
