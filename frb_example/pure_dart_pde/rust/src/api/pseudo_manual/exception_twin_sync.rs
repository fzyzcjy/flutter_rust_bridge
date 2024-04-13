// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::frb_generated::StreamSink;
use anyhow::{anyhow, Result};
use backtrace::Backtrace;
use flutter_rust_bridge::frb;

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

// ------------------------------ example-based ----------------------------------

pub enum CustomErrorTwinSync {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_err_custom_error_twin_sync() -> Result<u32, CustomErrorTwinSync> {
    Err(CustomErrorTwinSync::Error0 {
        e: "".into(),
        backtrace: Backtrace::new(),
    })
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_ok_custom_error_twin_sync() -> Result<u32, CustomErrorTwinSync> {
    Ok(3)
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_error_variant_twin_sync(variant: u32) -> Result<u32, CustomErrorTwinSync> {
    match variant {
        0 => Err(CustomErrorTwinSync::Error0 {
            e: "variant0".to_string(),
            backtrace: Backtrace::new(),
        }),
        1 => Err(CustomErrorTwinSync::Error1 {
            e: 1,
            backtrace: Backtrace::new(),
        }),
        _ => panic!("unsupported variant"),
    }
}

pub struct SomeStructTwinSync {
    pub value: u32,
}

impl SomeStructTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync(value: u32) -> SomeStructTwinSync {
        SomeStructTwinSync { value }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn static_return_err_custom_error_twin_sync() -> Result<u32, CustomErrorTwinSync> {
        Err(CustomErrorTwinSync::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn static_return_ok_custom_error_twin_sync() -> Result<u32, CustomErrorTwinSync> {
        Ok(3)
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn non_static_return_err_custom_error_twin_sync(&self) -> Result<u32, CustomErrorTwinSync> {
        Err(CustomErrorTwinSync::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn non_static_return_ok_custom_error_twin_sync(&self) -> Result<u32, CustomErrorTwinSync> {
        Ok(self.value)
    }
}

pub enum CustomNestedError1TwinSync {
    CustomNested1(String),
    ErrorNested(CustomNestedError2TwinSync),
}

pub enum CustomNestedError2TwinSync {
    CustomNested2(String),
    CustomNested2Number(u32),
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_nested_error_1_twin_sync() -> Result<(), CustomNestedError1TwinSync> {
    Err(CustomNestedError1TwinSync::ErrorNested(
        CustomNestedError2TwinSync::CustomNested2Number(3),
    ))
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_nested_error_1_variant1_twin_sync() -> Result<(), CustomNestedError1TwinSync> {
    Err(CustomNestedError1TwinSync::CustomNested1(
        "custom".to_string(),
    ))
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_nested_error_2_twin_sync() -> Result<(), CustomNestedError2TwinSync> {
    Err(CustomNestedError2TwinSync::CustomNested2(
        "custom".to_string(),
    ))
}
pub struct CustomStructErrorAnotherTwinSync {
    pub message: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_struct_error_twin_sync() -> Result<(), CustomStructErrorAnotherTwinSync> {
    Err(CustomStructErrorAnotherTwinSync {
        message: "error message".to_string(),
    })
}

#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_struct_ok_twin_sync() -> Result<u32, CustomStructErrorAnotherTwinSync> {
    Ok(3)
}

pub struct CustomStructTwinSync {
    pub message: String,
}

impl CustomStructTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync(message: String) -> CustomStructTwinSync {
        CustomStructTwinSync { message }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn static_return_custom_struct_error_twin_sync(
    ) -> Result<(), CustomStructErrorAnotherTwinSync> {
        Err(CustomStructErrorAnotherTwinSync {
            message: "error message".to_string(),
        })
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn static_return_custom_struct_ok_twin_sync(
    ) -> Result<u32, CustomStructErrorAnotherTwinSync> {
        Ok(3)
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn nonstatic_return_custom_struct_error_twin_sync(
        &self,
    ) -> Result<(), CustomStructErrorAnotherTwinSync> {
        Err(CustomStructErrorAnotherTwinSync {
            message: self.message.clone(),
        })
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn nonstatic_return_custom_struct_ok_twin_sync(
        &self,
    ) -> Result<u32, CustomStructErrorAnotherTwinSync> {
        Ok(3)
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn throw_anyhow_twin_sync() -> Result<(), anyhow::Error> {
    Err(anyhow!("anyhow error"))
}

#[flutter_rust_bridge::frb(sync)]
pub fn panic_with_custom_result_twin_sync() -> Result<(), CustomErrorTwinSync> {
    panic!("just a panic");
}

#[frb(stream_dart_await)]
#[flutter_rust_bridge::frb(sync)]
pub fn stream_sink_throw_anyhow_twin_sync(_sink: StreamSink<String>) -> Result<()> {
    Err(anyhow!("anyhow error"))
}
