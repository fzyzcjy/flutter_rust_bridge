// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;
use log::info;

#[frb]
#[derive(Debug, Clone)]
pub struct CustomizedTwinNormal {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

#[frb(ignore)]
pub fn func_should_not_exist_in_dart_twin_normal() {}

pub struct StructWithOnlyIgnoredMethodTwinNormal {}

impl StructWithOnlyIgnoredMethodTwinNormal {
    #[frb(ignore)]
    pub fn method_should_not_exist_in_dart_twin_normal(&self) {}
}

pub fn handle_customized_struct_twin_normal(val: CustomizedTwinNormal) {
    info!("{:#?}", val);
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserIdTwinNormal {
    #[frb(default = 0)]
    pub value: u32,
}

#[frb]
pub fn next_user_id_twin_normal(
    #[frb(default = "const UserIdTwinNormal()")] user_id: UserIdTwinNormal,
) -> UserIdTwinNormal {
    UserIdTwinNormal {
        value: user_id.value + 1,
    }
}

// Note: Some attributes are put on `KitchenSinkTwinNormal` currently
// (but we can add more tests in this file later)

#[frb(ignore)]
pub struct IgnoredStructTwinNormal {
    pub value: u32,
}

impl IgnoredStructTwinNormal {
    pub fn method_should_not_exist_in_dart_twin_normal(&self) {}
}
