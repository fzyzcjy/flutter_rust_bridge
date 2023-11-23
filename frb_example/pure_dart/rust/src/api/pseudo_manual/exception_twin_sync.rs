// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use anyhow::{anyhow, Result};

#[flutter_rust_bridge::frb(sync)]
pub fn func_return_error_twin_sync() -> Result<i32> {
    Err(anyhow!(
        "return_err() is called, thus deliberately return Err"
    ))
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_return_panic_twin_sync() -> i32 {
    panic!("return_panic() is called, thus deliberately panic")
}
