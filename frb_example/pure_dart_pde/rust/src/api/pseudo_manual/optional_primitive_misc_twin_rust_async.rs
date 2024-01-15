// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `optional_primitive_misc.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use log::info;

pub async fn primitive_optional_types_twin_rust_async(
    my_i32: Option<i32>,
    my_i64: Option<i64>,
    my_f64: Option<f64>,
    my_bool: Option<bool>,
) -> Option<i32> {
    info!(
        "primitive_optional_types({}, {}, {}, {})",
        my_i32.unwrap_or_default(),
        my_i64.unwrap_or_default(),
        my_f64.unwrap_or_default(),
        my_bool.unwrap_or_default()
    );
    Some(
        my_i32.is_some() as i32
            + my_i64.is_some() as i32
            + my_f64.is_some() as i32
            + my_bool.is_some() as i32,
    )
}
