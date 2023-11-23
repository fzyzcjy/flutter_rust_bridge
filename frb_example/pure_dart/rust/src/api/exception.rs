use anyhow::{anyhow, Result};
use backtrace::Backtrace;

// ------------------------------ built-in errors ----------------------------------

pub fn func_return_error_twin_normal() -> Result<i32> {
    Err(anyhow!("deliberate error"))
}

pub fn func_type_fallible_panic_twin_normal() -> Result<i32> {
    panic!("deliberate panic")
}

pub fn func_type_infallible_panic_twin_normal() -> i32 {
    panic!("deliberate panic")
}

// ------------------------------ custom error + return ok/panic ----------------------------------

pub fn custom_enum_error_return_ok_twin_normal(arg: u32) -> Result<u32, CustomEnumErrorTwinNormal> {
    Ok(arg)
}

pub fn custom_enum_error_panic_twin_normal() -> Result<(), CustomEnumErrorTwinNormal> {
    panic!("deliberate panic");
}

// ------------------------------ custom struct error ----------------------------------

pub enum CustomEnumErrorTwinNormal {
    One {
        message: String,
        backtrace: Backtrace,
    },
    Two {
        message: u32,
        backtrace: Backtrace,
    },
}

pub fn custom_enum_error_return_error_twin_normal() -> Result<u32, CustomEnumErrorTwinNormal> {
    Err(CustomEnumErrorTwinNormal::One {
        message: "deliberate error".into(),
        backtrace: Backtrace::new(),
    })
}

// ------------------------------ custom nested errors ----------------------------------

pub enum CustomNestedErrorOuterTwinNormal {
    One(String),
    Two(CustomNestedErrorInnerTwinNormal),
}

pub enum CustomNestedErrorInnerTwinNormal {
    Three(String),
    Four(u32),
}

pub fn custom_nested_error_return_error_twin_normal(
    arg: CustomNestedErrorOuterTwinNormal,
) -> Result<(), CustomNestedErrorOuterTwinNormal> {
    Err(arg)
}

// ------------------------------ custom struct errors ----------------------------------

pub struct CustomStructErrorTwinNormal {
    pub a: String,
}

pub fn custom_struct_error_return_error_twin_normal(
    arg: CustomStructErrorTwinNormal,
) -> Result<(), CustomStructErrorTwinNormal> {
    Err(arg)
}
