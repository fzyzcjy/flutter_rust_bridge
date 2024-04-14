// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::StreamSink;
use anyhow::{anyhow, Result};
use backtrace::Backtrace;
use flutter_rust_bridge::frb;

// ------------------------------ built-in errors ----------------------------------

#[flutter_rust_bridge::frb(serialize)]
pub fn func_return_error_twin_sse() -> Result<i32> {
    Err(anyhow!("deliberate error"))
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_type_fallible_panic_twin_sse() -> Result<i32> {
    panic!("deliberate panic")
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_type_infallible_panic_twin_sse() -> i32 {
    panic!("deliberate panic")
}

// ------------------------------ custom error + return ok/panic ----------------------------------

#[flutter_rust_bridge::frb(serialize)]
pub fn custom_enum_error_return_ok_twin_sse(arg: u32) -> Result<u32, CustomEnumErrorTwinSse> {
    Ok(arg)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn custom_enum_error_panic_twin_sse() -> Result<(), CustomEnumErrorTwinSse> {
    panic!("deliberate panic");
}

// ------------------------------ custom struct error ----------------------------------

pub enum CustomEnumErrorTwinSse {
    One {
        message: String,
        backtrace: Backtrace,
    },
    Two {
        message: u32,
        backtrace: Backtrace,
    },
}

#[flutter_rust_bridge::frb(serialize)]
pub fn custom_enum_error_return_error_twin_sse() -> Result<u32, CustomEnumErrorTwinSse> {
    Err(CustomEnumErrorTwinSse::One {
        message: "deliberate error".into(),
        backtrace: Backtrace::new(),
    })
}

// ------------------------------ custom nested errors ----------------------------------

pub enum CustomNestedErrorOuterTwinSse {
    One(String),
    Two(CustomNestedErrorInnerTwinSse),
}

pub enum CustomNestedErrorInnerTwinSse {
    Three(String),
    Four(u32),
}

#[flutter_rust_bridge::frb(serialize)]
pub fn custom_nested_error_return_error_twin_sse(
    arg: CustomNestedErrorOuterTwinSse,
) -> Result<(), CustomNestedErrorOuterTwinSse> {
    Err(arg)
}

// ------------------------------ custom struct errors ----------------------------------

pub struct CustomStructErrorTwinSse {
    pub a: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn custom_struct_error_return_error_twin_sse(
    arg: CustomStructErrorTwinSse,
) -> Result<(), CustomStructErrorTwinSse> {
    Err(arg)
}

// ------------------------------ example-based ----------------------------------

pub enum CustomErrorTwinSse {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_err_custom_error_twin_sse() -> Result<u32, CustomErrorTwinSse> {
    Err(CustomErrorTwinSse::Error0 {
        e: "".into(),
        backtrace: Backtrace::new(),
    })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_ok_custom_error_twin_sse() -> Result<u32, CustomErrorTwinSse> {
    Ok(3)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_error_variant_twin_sse(variant: u32) -> Result<u32, CustomErrorTwinSse> {
    match variant {
        0 => Err(CustomErrorTwinSse::Error0 {
            e: "variant0".to_string(),
            backtrace: Backtrace::new(),
        }),
        1 => Err(CustomErrorTwinSse::Error1 {
            e: 1,
            backtrace: Backtrace::new(),
        }),
        _ => panic!("unsupported variant"),
    }
}

pub struct SomeStructTwinSse {
    pub value: u32,
}

impl SomeStructTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse(value: u32) -> SomeStructTwinSse {
        SomeStructTwinSse { value }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_return_err_custom_error_twin_sse() -> Result<u32, CustomErrorTwinSse> {
        Err(CustomErrorTwinSse::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_return_ok_custom_error_twin_sse() -> Result<u32, CustomErrorTwinSse> {
        Ok(3)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn non_static_return_err_custom_error_twin_sse(&self) -> Result<u32, CustomErrorTwinSse> {
        Err(CustomErrorTwinSse::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn non_static_return_ok_custom_error_twin_sse(&self) -> Result<u32, CustomErrorTwinSse> {
        Ok(self.value)
    }
}

pub enum CustomNestedError1TwinSse {
    CustomNested1(String),
    ErrorNested(CustomNestedError2TwinSse),
}

pub enum CustomNestedError2TwinSse {
    CustomNested2(String),
    CustomNested2Number(u32),
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_custom_nested_error_1_twin_sse() -> Result<(), CustomNestedError1TwinSse> {
    Err(CustomNestedError1TwinSse::ErrorNested(
        CustomNestedError2TwinSse::CustomNested2Number(3),
    ))
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_custom_nested_error_1_variant1_twin_sse() -> Result<(), CustomNestedError1TwinSse> {
    Err(CustomNestedError1TwinSse::CustomNested1(
        "custom".to_string(),
    ))
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_custom_nested_error_2_twin_sse() -> Result<(), CustomNestedError2TwinSse> {
    Err(CustomNestedError2TwinSse::CustomNested2(
        "custom".to_string(),
    ))
}
pub struct CustomStructErrorAnotherTwinSse {
    pub message: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_custom_struct_error_twin_sse() -> Result<(), CustomStructErrorAnotherTwinSse> {
    Err(CustomStructErrorAnotherTwinSse {
        message: "error message".to_string(),
    })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn return_custom_struct_ok_twin_sse() -> Result<u32, CustomStructErrorAnotherTwinSse> {
    Ok(3)
}

pub struct CustomStructTwinSse {
    pub message: String,
}

impl CustomStructTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse(message: String) -> CustomStructTwinSse {
        CustomStructTwinSse { message }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_return_custom_struct_error_twin_sse(
    ) -> Result<(), CustomStructErrorAnotherTwinSse> {
        Err(CustomStructErrorAnotherTwinSse {
            message: "error message".to_string(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_return_custom_struct_ok_twin_sse() -> Result<u32, CustomStructErrorAnotherTwinSse>
    {
        Ok(3)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn nonstatic_return_custom_struct_error_twin_sse(
        &self,
    ) -> Result<(), CustomStructErrorAnotherTwinSse> {
        Err(CustomStructErrorAnotherTwinSse {
            message: self.message.clone(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn nonstatic_return_custom_struct_ok_twin_sse(
        &self,
    ) -> Result<u32, CustomStructErrorAnotherTwinSse> {
        Ok(3)
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn throw_anyhow_twin_sse() -> Result<(), anyhow::Error> {
    Err(anyhow!("anyhow error"))
}

#[flutter_rust_bridge::frb(serialize)]
pub fn panic_with_custom_result_twin_sse() -> Result<(), CustomErrorTwinSse> {
    panic!("just a panic");
}

#[frb(stream_dart_await)]
#[flutter_rust_bridge::frb(serialize)]
pub fn stream_sink_throw_anyhow_twin_sse(
    _sink: StreamSink<String, flutter_rust_bridge::SseCodec>,
) -> Result<()> {
    Err(anyhow!("anyhow error"))
}
