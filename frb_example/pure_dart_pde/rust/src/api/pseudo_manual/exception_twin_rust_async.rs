// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::frb_generated::StreamSink;
use anyhow::{anyhow, Result};
use backtrace::Backtrace;
use flutter_rust_bridge::frb;

// ------------------------------ built-in errors ----------------------------------

pub async fn func_return_error_twin_rust_async() -> Result<i32> {
    Err(anyhow!("deliberate error"))
}

pub async fn func_type_fallible_panic_twin_rust_async() -> Result<i32> {
    panic!("deliberate panic")
}

pub async fn func_type_infallible_panic_twin_rust_async() -> i32 {
    panic!("deliberate panic")
}

// ------------------------------ custom error + return ok/panic ----------------------------------

pub async fn custom_enum_error_return_ok_twin_rust_async(
    arg: u32,
) -> Result<u32, CustomEnumErrorTwinRustAsync> {
    Ok(arg)
}

pub async fn custom_enum_error_panic_twin_rust_async() -> Result<(), CustomEnumErrorTwinRustAsync> {
    panic!("deliberate panic");
}

// ------------------------------ custom struct error ----------------------------------

pub enum CustomEnumErrorTwinRustAsync {
    One {
        message: String,
        backtrace: Backtrace,
    },
    Two {
        message: u32,
        backtrace: Backtrace,
    },
}

pub async fn custom_enum_error_return_error_twin_rust_async(
) -> Result<u32, CustomEnumErrorTwinRustAsync> {
    Err(CustomEnumErrorTwinRustAsync::One {
        message: "deliberate error".into(),
        backtrace: Backtrace::new(),
    })
}

// ------------------------------ custom nested errors ----------------------------------

pub enum CustomNestedErrorOuterTwinRustAsync {
    One(String),
    Two(CustomNestedErrorInnerTwinRustAsync),
}

pub enum CustomNestedErrorInnerTwinRustAsync {
    Three(String),
    Four(u32),
}

pub async fn custom_nested_error_return_error_twin_rust_async(
    arg: CustomNestedErrorOuterTwinRustAsync,
) -> Result<(), CustomNestedErrorOuterTwinRustAsync> {
    Err(arg)
}

// ------------------------------ custom struct errors ----------------------------------

pub struct CustomStructErrorTwinRustAsync {
    pub a: String,
}

pub async fn custom_struct_error_return_error_twin_rust_async(
    arg: CustomStructErrorTwinRustAsync,
) -> Result<(), CustomStructErrorTwinRustAsync> {
    Err(arg)
}

// ------------------------------ example-based ----------------------------------

pub enum CustomErrorTwinRustAsync {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}

pub async fn return_err_custom_error_twin_rust_async() -> Result<u32, CustomErrorTwinRustAsync> {
    Err(CustomErrorTwinRustAsync::Error0 {
        e: "".into(),
        backtrace: Backtrace::new(),
    })
}

pub async fn return_ok_custom_error_twin_rust_async() -> Result<u32, CustomErrorTwinRustAsync> {
    Ok(3)
}

pub async fn return_error_variant_twin_rust_async(
    variant: u32,
) -> Result<u32, CustomErrorTwinRustAsync> {
    match variant {
        0 => Err(CustomErrorTwinRustAsync::Error0 {
            e: "variant0".to_string(),
            backtrace: Backtrace::new(),
        }),
        1 => Err(CustomErrorTwinRustAsync::Error1 {
            e: 1,
            backtrace: Backtrace::new(),
        }),
        _ => panic!("unsupported variant"),
    }
}

pub struct SomeStructTwinRustAsync {
    pub value: u32,
}

impl SomeStructTwinRustAsync {
    pub async fn new_twin_rust_async(value: u32) -> SomeStructTwinRustAsync {
        SomeStructTwinRustAsync { value }
    }

    pub async fn static_return_err_custom_error_twin_rust_async(
    ) -> Result<u32, CustomErrorTwinRustAsync> {
        Err(CustomErrorTwinRustAsync::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    pub async fn static_return_ok_custom_error_twin_rust_async(
    ) -> Result<u32, CustomErrorTwinRustAsync> {
        Ok(3)
    }

    pub async fn non_static_return_err_custom_error_twin_rust_async(
        &self,
    ) -> Result<u32, CustomErrorTwinRustAsync> {
        Err(CustomErrorTwinRustAsync::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    pub async fn non_static_return_ok_custom_error_twin_rust_async(
        &self,
    ) -> Result<u32, CustomErrorTwinRustAsync> {
        Ok(self.value)
    }
}

pub enum CustomNestedError1TwinRustAsync {
    CustomNested1(String),
    ErrorNested(CustomNestedError2TwinRustAsync),
}

pub enum CustomNestedError2TwinRustAsync {
    CustomNested2(String),
    CustomNested2Number(u32),
}

pub async fn return_custom_nested_error_1_twin_rust_async(
) -> Result<(), CustomNestedError1TwinRustAsync> {
    Err(CustomNestedError1TwinRustAsync::ErrorNested(
        CustomNestedError2TwinRustAsync::CustomNested2Number(3),
    ))
}

pub async fn return_custom_nested_error_1_variant1_twin_rust_async(
) -> Result<(), CustomNestedError1TwinRustAsync> {
    Err(CustomNestedError1TwinRustAsync::CustomNested1(
        "custom".to_string(),
    ))
}

pub async fn return_custom_nested_error_2_twin_rust_async(
) -> Result<(), CustomNestedError2TwinRustAsync> {
    Err(CustomNestedError2TwinRustAsync::CustomNested2(
        "custom".to_string(),
    ))
}
pub struct CustomStructErrorAnotherTwinRustAsync {
    pub message: String,
}

pub async fn return_custom_struct_error_twin_rust_async(
) -> Result<(), CustomStructErrorAnotherTwinRustAsync> {
    Err(CustomStructErrorAnotherTwinRustAsync {
        message: "error message".to_string(),
    })
}

pub async fn return_custom_struct_ok_twin_rust_async(
) -> Result<u32, CustomStructErrorAnotherTwinRustAsync> {
    Ok(3)
}

pub struct CustomStructTwinRustAsync {
    pub message: String,
}

impl CustomStructTwinRustAsync {
    pub async fn new_twin_rust_async(message: String) -> CustomStructTwinRustAsync {
        CustomStructTwinRustAsync { message }
    }

    pub async fn static_return_custom_struct_error_twin_rust_async(
    ) -> Result<(), CustomStructErrorAnotherTwinRustAsync> {
        Err(CustomStructErrorAnotherTwinRustAsync {
            message: "error message".to_string(),
        })
    }

    pub async fn static_return_custom_struct_ok_twin_rust_async(
    ) -> Result<u32, CustomStructErrorAnotherTwinRustAsync> {
        Ok(3)
    }

    pub async fn nonstatic_return_custom_struct_error_twin_rust_async(
        &self,
    ) -> Result<(), CustomStructErrorAnotherTwinRustAsync> {
        Err(CustomStructErrorAnotherTwinRustAsync {
            message: self.message.clone(),
        })
    }

    pub async fn nonstatic_return_custom_struct_ok_twin_rust_async(
        &self,
    ) -> Result<u32, CustomStructErrorAnotherTwinRustAsync> {
        Ok(3)
    }
}

pub async fn throw_anyhow_twin_rust_async() -> Result<(), anyhow::Error> {
    Err(anyhow!("anyhow error"))
}

pub async fn panic_with_custom_result_twin_rust_async() -> Result<(), CustomErrorTwinRustAsync> {
    panic!("just a panic");
}

#[frb(stream_dart_await)]
pub async fn stream_sink_throw_anyhow_twin_rust_async(_sink: StreamSink<String>) -> Result<()> {
    Err(anyhow!("anyhow error"))
}
