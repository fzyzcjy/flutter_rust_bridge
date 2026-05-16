pub fn test_tuple_twin_normal(value: Option<(String, i32)>) -> (String, i32) {
    if let Some((name, value)) = value {
        (format!("Hello {name}"), value + 1)
    } else {
        ("John".to_string(), 0)
    }
}

pub fn test_tuple_2_twin_normal(value: Vec<(String, i32)>) {
    drop(value)
}

pub fn return_optional_f_32_tuple_twin_normal() -> Option<(f32, f32)> {
    Some((1.25, 2.5))
}
