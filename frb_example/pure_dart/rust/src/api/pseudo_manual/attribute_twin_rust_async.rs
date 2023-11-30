// NOTE: This file is mimicking how a human developer writes tests, 
// and is auto-generated from `attribute.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::api::pseudo_manual::misc_example_twin_rust_async::WeekdaysTwinRustAsync;
use flutter_rust_bridge::{frb, ZeroCopyBuffer};
use log::info;

#[frb]
#[derive(Debug, Clone)]
pub struct CustomizedTwinRustAsync {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

pub async fn handle_customized_struct_twin_rust_async(val: CustomizedTwinRustAsync) {
    info!("{:#?}", val);
}

#[frb]
#[derive(Debug)]
pub enum KitchenSinkTwinRustAsync {
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
        #[frb(default = "KitchenSinkTwinRustAsync.empty()")] Box<KitchenSinkTwinRustAsync>,
    ),
    Optional(#[frb(default = -1)] Option<i32>, Option<i32>),
    Buffer(ZeroCopyBuffer<Vec<u8>>),
    Enums(#[frb(default = "WeekdaysTwinRustAsync.Sunday")] WeekdaysTwinRustAsync),
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserIdTwinRustAsync {
    #[frb(default = 0)]
    pub value: u32,
}

pub async fn next_user_id_twin_rust_async(user_id: UserIdTwinRustAsync) -> UserIdTwinRustAsync {
    UserIdTwinRustAsync {
        value: user_id.value + 1,
    }
}
