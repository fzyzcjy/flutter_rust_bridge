pub fn test_tuple(value: Option<(String, i32)>) -> (String, i32) {
    if let Some((name, value)) = value {
        (format!("Hello {name}"), value + 1)
    } else {
        ("John".to_string(), 0)
    }
}

pub fn test_tuple_2(value: Vec<(String, i32)>) {
    drop(value)
}
