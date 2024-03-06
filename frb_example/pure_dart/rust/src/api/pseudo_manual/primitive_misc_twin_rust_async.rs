// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `primitive_misc.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use log::info;

pub async fn primitive_types_twin_rust_async(
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) -> i32 {
    info!(
        "primitive_types({}, {}, {}, {})",
        my_i32, my_i64, my_f64, my_bool
    );
    42
}

pub async fn primitive_u32_twin_rust_async(my_u32: u32) -> u32 {
    info!("primitive_u32({})", my_u32);
    assert_eq!(my_u32, 0xff112233);
    let ret = 0xfe112233;
    info!("returning {}", ret);
    ret
}
