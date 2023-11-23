use anyhow::{anyhow, Result};

pub fn func_return_error_twin_normal() -> Result<i32> {
    Err(anyhow!(
        "return_err() is called, thus deliberately return Err"
    ))
}

pub fn func_return_panic_twin_normal() -> i32 {
    panic!("return_panic() is called, thus deliberately panic")
}
