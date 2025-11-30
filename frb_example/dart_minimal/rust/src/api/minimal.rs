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

pub fn fallible_divide(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError { message: "division by zero".to_string() })
    } else {
        Ok(a / b)
    }
}

// Sync function test
#[frb(sync)]
pub fn fallible_divide_sync(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 {
        Err(MyError { message: "division by zero".to_string() })
    } else {
        Ok(a / b)
    }
}
