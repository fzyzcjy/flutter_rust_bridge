use log::info;

pub fn primitive_types_twin_normal(my_i32: i32, my_i64: i64, my_f64: f64, my_bool: bool) -> i32 {
    info!(
        "primitive_types({}, {}, {}, {})",
        my_i32, my_i64, my_f64, my_bool
    );
    42
}

pub fn primitive_u32_twin_normal(my_u32: u32) -> u32 {
    info!("primitive_u32({})", my_u32);
    assert_eq!(my_u32, 0xff112233);
    let ret = 0xfe112233;
    info!("returning {}", ret);
    ret
}
