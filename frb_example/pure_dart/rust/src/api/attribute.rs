use crate::api::misc_example::WeekdaysTwinNormal;
use flutter_rust_bridge::{frb, ZeroCopyBuffer};
use log::info;

#[frb]
#[derive(Debug, Clone)]
pub struct CustomizedTwinNormal {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

pub fn handle_customized_struct_twin_normal(val: CustomizedTwinNormal) {
    info!("{:#?}", val);
}

#[frb]
#[derive(Debug)]
pub enum KitchenSinkTwinNormal {
    Empty,
    #[frb(unimpl_variant_attr)]
    Primitives {
        #[frb(default = -1)]
        int32: i32,
        float64: f64,
        boolean: bool,
    },
    Nested(
        i32,
        #[frb(default = "KitchenSinkTwinNormal.empty()")] Box<KitchenSinkTwinNormal>,
    ),
    Optional(#[frb(default = -1)] Option<i32>, Option<i32>),
    Buffer(ZeroCopyBuffer<Vec<u8>>),
    Enums(#[frb(default = "WeekdaysTwinNormal.Sunday")] WeekdaysTwinNormal),
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserIdTwinNormal {
    #[frb(default = 0)]
    pub value: u32,
}

pub fn next_user_id_twin_normal(user_id: UserIdTwinNormal) -> UserIdTwinNormal {
    UserId {
        value: user_id.value + 1,
    }
}
