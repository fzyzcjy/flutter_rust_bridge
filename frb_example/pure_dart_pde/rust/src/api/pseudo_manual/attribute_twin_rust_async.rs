// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `attribute.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;
use log::info;

#[frb]
#[derive(Debug, Clone)]
pub struct CustomizedTwinRustAsync {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

#[frb(ignore)]
pub async fn func_should_not_exist_in_dart_twin_rust_async() {}

pub struct StructWithOnlyIgnoredMethodTwinRustAsync {}

impl StructWithOnlyIgnoredMethodTwinRustAsync {
    #[frb(ignore)]
    pub async fn method_should_not_exist_in_dart_twin_rust_async(&self) {}
}

pub async fn handle_customized_struct_twin_rust_async(val: CustomizedTwinRustAsync) {
    info!("{:#?}", val);
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserIdTwinRustAsync {
    #[frb(default = 0)]
    pub value: u32,
}

#[frb]
pub async fn next_user_id_twin_rust_async(
    #[frb(default = "const UserIdTwinRustAsync()")] user_id: UserIdTwinRustAsync,
) -> UserIdTwinRustAsync {
    UserIdTwinRustAsync {
        value: user_id.value + 1,
    }
}

// Note: Some attributes are put on `KitchenSinkTwinRustAsync` currently
// (but we can add more tests in this file later)

#[frb(ignore)]
pub struct IgnoredStructTwinRustAsync {
    pub value: u32,
}

impl IgnoredStructTwinRustAsync {
    pub async fn method_should_not_exist_in_dart_twin_rust_async(&self) {}
}
