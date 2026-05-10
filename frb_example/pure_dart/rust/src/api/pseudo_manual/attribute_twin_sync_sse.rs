// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `attribute.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;
use log::info;

#[frb]
#[derive(Debug, Clone)]
pub struct CustomizedTwinSyncSse {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

#[frb(ignore)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_should_not_exist_in_dart_twin_sync_sse() {}

pub struct StructWithOnlyIgnoredMethodTwinSyncSse {}

impl StructWithOnlyIgnoredMethodTwinSyncSse {
    #[frb(ignore)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn method_should_not_exist_in_dart_twin_sync_sse(&self) {}
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_customized_struct_twin_sync_sse(val: CustomizedTwinSyncSse) {
    info!("{:#?}", val);
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserIdTwinSyncSse {
    #[frb(default = 0)]
    pub value: u32,
}

#[frb]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn next_user_id_twin_sync_sse(
    #[frb(default = "const UserIdTwinSyncSse()")] user_id: UserIdTwinSyncSse,
) -> UserIdTwinSyncSse {
    UserIdTwinSyncSse {
        value: user_id.value + 1,
    }
}

// Note: Some attributes are put on `KitchenSinkTwinSyncSse` currently
// (but we can add more tests in this file later)

#[frb(ignore)]
pub struct IgnoredStructTwinSyncSse {
    pub value: u32,
}

impl IgnoredStructTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn method_should_not_exist_in_dart_twin_sync_sse(&self) {}
}
