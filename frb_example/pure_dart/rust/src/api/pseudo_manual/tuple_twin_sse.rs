// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `tuple.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[flutter_rust_bridge::frb(serialize)]
pub fn test_tuple_twin_sse(value: Option<(String, i32)>) -> (String, i32) {
    if let Some((name, value)) = value {
        (format!("Hello {name}"), value + 1)
    } else {
        ("John".to_string(), 0)
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn test_tuple_2_twin_sse(value: Vec<(String, i32)>) {
    drop(value)
}
