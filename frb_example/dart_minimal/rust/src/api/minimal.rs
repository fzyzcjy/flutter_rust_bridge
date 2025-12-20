use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// Oxidized Result support test
pub struct MyError {
    pub message: String,
}

#[frb(oxidized)]
pub fn fallible_divide(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError { message: "division by zero".to_string() })
    } else {
        Ok(a / b)
    }
}

/// This function throws an exception (no #[frb(oxidized)])
pub fn fallible_divide_throws(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError { message: "division by zero".to_string() })
    } else {
        Ok(a / b)
    }
}

// Sync function test
#[frb(oxidized)]
#[frb(sync)]
pub fn fallible_divide_sync(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError { message: "division by zero".to_string() })
    } else {
        Ok(a / b)
    }
}

// =============================================================================
// Generic type alias tests
// =============================================================================

/// Custom Result type alias with fixed error type
pub type WResult<T> = std::result::Result<T, MyError>;

/// Test basic WResult alias with primitive type
#[frb(oxidized)]
#[frb(sync)]
pub fn test_wresult_alias(a: i32, b: i32) -> WResult<i32> {
    if b == 0 {
        Err(MyError { message: "division by zero".to_string() })
    } else {
        Ok(a / b)
    }
}

/// Test WResult alias with UUID type
#[frb(oxidized)]
pub fn test_wresult_uuid() -> WResult<uuid::Uuid> {
    Ok(uuid::Uuid::new_v4())
}

/// Test WResult alias with String type
#[frb(oxidized)]
#[frb(sync)]
pub fn test_wresult_string(name: String) -> WResult<String> {
    if name.is_empty() {
        Err(MyError { message: "name cannot be empty".to_string() })
    } else {
        Ok(format!("Hello, {}!", name))
    }
}

/// Custom struct for testing
pub struct UserInfo {
    pub id: i32,
    pub name: String,
}

/// Test WResult alias with custom struct type
#[frb(oxidized)]
pub fn test_wresult_struct(id: i32, name: String) -> WResult<UserInfo> {
    if id < 0 {
        Err(MyError { message: "id must be non-negative".to_string() })
    } else {
        Ok(UserInfo { id, name })
    }
}

/// Test WResult alias with Vec type
#[frb(oxidized)]
pub fn test_wresult_vec(count: i32) -> WResult<Vec<i32>> {
    if count < 0 {
        Err(MyError { message: "count must be non-negative".to_string() })
    } else {
        Ok((0..count).collect())
    }
}

/// Test WResult alias with Option type
#[frb(oxidized)]
pub fn test_wresult_option(value: Option<i32>) -> WResult<Option<String>> {
    match value {
        Some(v) if v < 0 => Err(MyError { message: "value must be non-negative".to_string() }),
        Some(v) => Ok(Some(format!("Value: {}", v))),
        None => Ok(None),
    }
}

// =============================================================================
// Generic type alias with multiple parameters
// =============================================================================

/// Type alias with two generic parameters
pub type MyPair<A, B> = (A, B);

/// Test two-parameter generic alias
#[frb(sync)]
pub fn test_pair_alias(a: i32, b: String) -> MyPair<i32, String> {
    (a, b)
}

// =============================================================================
// Nested generic type alias
// =============================================================================

/// Nested type alias: WResult containing a Vec
#[frb(oxidized)]
pub fn test_wresult_nested(items: Vec<String>) -> WResult<Vec<String>> {
    if items.is_empty() {
        Err(MyError { message: "items cannot be empty".to_string() })
    } else {
        Ok(items.into_iter().map(|s| s.to_uppercase()).collect())
    }
}
