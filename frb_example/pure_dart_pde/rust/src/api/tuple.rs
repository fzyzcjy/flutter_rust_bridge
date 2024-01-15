// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

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
