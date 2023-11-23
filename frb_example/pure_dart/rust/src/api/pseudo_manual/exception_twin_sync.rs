// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use anyhow::{anyhow, Result};
use backtrace::Backtrace;

// ------------------------------ built-in errors ----------------------------------

#[flutter_rust_bridge::frb(sync)]
pub fn func_return_error_twin_sync() -> Result<i32> {
    Err(anyhow!("deliberate error"))
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_type_fallible_panic_twin_sync() -> Result<i32> {
    panic!("deliberate panic")
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_type_infallible_panic_twin_sync() -> i32 {
    panic!("deliberate panic")
}

// ------------------------------ custom error + return ok/panic ----------------------------------

#[flutter_rust_bridge::frb(sync)]
pub fn custom_enum_error_return_ok_twin_sync(arg: u32) -> Result<u32, CustomEnumErrorTwinSync> {
    Ok(arg)
}

#[flutter_rust_bridge::frb(sync)]
pub fn custom_enum_error_panic_twin_sync() -> Result<(), CustomEnumErrorTwinSync> {
    panic!("deliberate panic");
}

// ------------------------------ custom struct error ----------------------------------

pub enum CustomEnumErrorTwinSync {
    One {
        message: String,
        backtrace: Backtrace,
    },
    Two {
        message: u32,
        backtrace: Backtrace,
    },
}

#[flutter_rust_bridge::frb(sync)]
pub fn custom_enum_error_return_error_twin_sync() -> Result<u32, CustomEnumErrorTwinSync> {
    Err(CustomEnumErrorTwinSync::One {
        message: "deliberate error".into(),
        backtrace: Backtrace::new(),
    })
}

// ------------------------------ custom nested errors ----------------------------------

pub enum CustomNestedErrorOuterTwinSync {
    One(String),
    Two(CustomNestedErrorInnerTwinSync),
}

pub enum CustomNestedErrorInnerTwinSync {
    Three(String),
    Four(u32),
}

#[flutter_rust_bridge::frb(sync)]
pub fn custom_nested_error_return_error_twin_sync(
    arg: CustomNestedErrorOuterTwinSync,
) -> Result<(), CustomNestedErrorOuterTwinSync> {
    Err(arg)
}

// ------------------------------ custom struct errors ----------------------------------

pub struct CustomStructErrorTwinSync {
    pub a: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn custom_struct_error_return_error_twin_sync(
    arg: CustomStructErrorTwinSync,
) -> Result<(), CustomStructErrorTwinSync> {
    Err(arg)
}
