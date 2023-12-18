// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `attribute.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;
use log::info;

#[frb]
#[derive(Debug, Clone)]
pub struct CustomizedTwinSse {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_customized_struct_twin_sse(val: CustomizedTwinSse) {
    info!("{:#?}", val);
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserIdTwinSse {
    #[frb(default = 0)]
    pub value: u32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn next_user_id_twin_sse(user_id: UserIdTwinSse) -> UserIdTwinSse {
    UserIdTwinSse {
        value: user_id.value + 1,
    }
}

// Note: Some attributes are put on `KitchenSinkTwinSse` currently
// (but we can add more tests in this file later)
