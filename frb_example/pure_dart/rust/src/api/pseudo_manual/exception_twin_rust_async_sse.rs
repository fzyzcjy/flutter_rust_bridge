// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::StreamSink;
use anyhow::{anyhow, Result};
use backtrace::Backtrace;
use flutter_rust_bridge::frb;

// ------------------------------ built-in errors ----------------------------------

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_return_error_twin_rust_async_sse() -> Result<i32> {
    Err(anyhow!("deliberate error"))
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_type_fallible_panic_twin_rust_async_sse() -> Result<i32> {
    panic!("deliberate panic")
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_type_infallible_panic_twin_rust_async_sse() -> i32 {
    panic!("deliberate panic")
}

// ------------------------------ custom error + return ok/panic ----------------------------------

#[flutter_rust_bridge::frb(serialize)]
pub async fn custom_enum_error_return_ok_twin_rust_async_sse(
    arg: u32,
) -> Result<u32, CustomEnumErrorTwinRustAsyncSse> {
    Ok(arg)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn custom_enum_error_panic_twin_rust_async_sse(
) -> Result<(), CustomEnumErrorTwinRustAsyncSse> {
    panic!("deliberate panic");
}

// ------------------------------ custom struct error ----------------------------------

pub enum CustomEnumErrorTwinRustAsyncSse {
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
pub async fn custom_enum_error_return_error_twin_rust_async_sse(
) -> Result<u32, CustomEnumErrorTwinRustAsyncSse> {
    Err(CustomEnumErrorTwinRustAsyncSse::One {
        message: "deliberate error".into(),
        backtrace: Backtrace::new(),
    })
}

// ------------------------------ custom nested errors ----------------------------------

pub enum CustomNestedErrorOuterTwinRustAsyncSse {
    One(String),
    Two(CustomNestedErrorInnerTwinRustAsyncSse),
}

pub enum CustomNestedErrorInnerTwinRustAsyncSse {
    Three(String),
    Four(u32),
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn custom_nested_error_return_error_twin_rust_async_sse(
    arg: CustomNestedErrorOuterTwinRustAsyncSse,
) -> Result<(), CustomNestedErrorOuterTwinRustAsyncSse> {
    Err(arg)
}

// ------------------------------ custom struct errors ----------------------------------

pub struct CustomStructErrorTwinRustAsyncSse {
    pub a: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn custom_struct_error_return_error_twin_rust_async_sse(
    arg: CustomStructErrorTwinRustAsyncSse,
) -> Result<(), CustomStructErrorTwinRustAsyncSse> {
    Err(arg)
}

// ------------------------------ example-based ----------------------------------

pub enum CustomErrorTwinRustAsyncSse {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_err_custom_error_twin_rust_async_sse(
) -> Result<u32, CustomErrorTwinRustAsyncSse> {
    Err(CustomErrorTwinRustAsyncSse::Error0 {
        e: "".into(),
        backtrace: Backtrace::new(),
    })
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_ok_custom_error_twin_rust_async_sse() -> Result<u32, CustomErrorTwinRustAsyncSse>
{
    Ok(3)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_error_variant_twin_rust_async_sse(
    variant: u32,
) -> Result<u32, CustomErrorTwinRustAsyncSse> {
    match variant {
        0 => Err(CustomErrorTwinRustAsyncSse::Error0 {
            e: "variant0".to_string(),
            backtrace: Backtrace::new(),
        }),
        1 => Err(CustomErrorTwinRustAsyncSse::Error1 {
            e: 1,
            backtrace: Backtrace::new(),
        }),
        _ => panic!("unsupported variant"),
    }
}

pub struct SomeStructTwinRustAsyncSse {
    pub value: u32,
}

impl SomeStructTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_twin_rust_async_sse(value: u32) -> SomeStructTwinRustAsyncSse {
        SomeStructTwinRustAsyncSse { value }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_return_err_custom_error_twin_rust_async_sse(
    ) -> Result<u32, CustomErrorTwinRustAsyncSse> {
        Err(CustomErrorTwinRustAsyncSse::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_return_ok_custom_error_twin_rust_async_sse(
    ) -> Result<u32, CustomErrorTwinRustAsyncSse> {
        Ok(3)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn non_static_return_err_custom_error_twin_rust_async_sse(
        &self,
    ) -> Result<u32, CustomErrorTwinRustAsyncSse> {
        Err(CustomErrorTwinRustAsyncSse::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn non_static_return_ok_custom_error_twin_rust_async_sse(
        &self,
    ) -> Result<u32, CustomErrorTwinRustAsyncSse> {
        Ok(self.value)
    }
}

pub enum CustomNestedError1TwinRustAsyncSse {
    CustomNested1(String),
    ErrorNested(CustomNestedError2TwinRustAsyncSse),
}

pub enum CustomNestedError2TwinRustAsyncSse {
    CustomNested2(String),
    CustomNested2Number(u32),
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_custom_nested_error_1_twin_rust_async_sse(
) -> Result<(), CustomNestedError1TwinRustAsyncSse> {
    Err(CustomNestedError1TwinRustAsyncSse::ErrorNested(
        CustomNestedError2TwinRustAsyncSse::CustomNested2Number(3),
    ))
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_custom_nested_error_1_variant1_twin_rust_async_sse(
) -> Result<(), CustomNestedError1TwinRustAsyncSse> {
    Err(CustomNestedError1TwinRustAsyncSse::CustomNested1(
        "custom".to_string(),
    ))
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_custom_nested_error_2_twin_rust_async_sse(
) -> Result<(), CustomNestedError2TwinRustAsyncSse> {
    Err(CustomNestedError2TwinRustAsyncSse::CustomNested2(
        "custom".to_string(),
    ))
}
pub struct CustomStructErrorAnotherTwinRustAsyncSse {
    pub message: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_custom_struct_error_twin_rust_async_sse(
) -> Result<(), CustomStructErrorAnotherTwinRustAsyncSse> {
    Err(CustomStructErrorAnotherTwinRustAsyncSse {
        message: "error message".to_string(),
    })
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn return_custom_struct_ok_twin_rust_async_sse(
) -> Result<u32, CustomStructErrorAnotherTwinRustAsyncSse> {
    Ok(3)
}

pub struct CustomStructTwinRustAsyncSse {
    pub message: String,
}

impl CustomStructTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_twin_rust_async_sse(message: String) -> CustomStructTwinRustAsyncSse {
        CustomStructTwinRustAsyncSse { message }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_return_custom_struct_error_twin_rust_async_sse(
    ) -> Result<(), CustomStructErrorAnotherTwinRustAsyncSse> {
        Err(CustomStructErrorAnotherTwinRustAsyncSse {
            message: "error message".to_string(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_return_custom_struct_ok_twin_rust_async_sse(
    ) -> Result<u32, CustomStructErrorAnotherTwinRustAsyncSse> {
        Ok(3)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn nonstatic_return_custom_struct_error_twin_rust_async_sse(
        &self,
    ) -> Result<(), CustomStructErrorAnotherTwinRustAsyncSse> {
        Err(CustomStructErrorAnotherTwinRustAsyncSse {
            message: self.message.clone(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn nonstatic_return_custom_struct_ok_twin_rust_async_sse(
        &self,
    ) -> Result<u32, CustomStructErrorAnotherTwinRustAsyncSse> {
        Ok(3)
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn throw_anyhow_twin_rust_async_sse() -> Result<(), anyhow::Error> {
    Err(anyhow!("anyhow error"))
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn panic_with_custom_result_twin_rust_async_sse(
) -> Result<(), CustomErrorTwinRustAsyncSse> {
    panic!("just a panic");
}

#[frb(stream_dart_await)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn stream_sink_throw_anyhow_twin_rust_async_sse(
    _sink: StreamSink<String, flutter_rust_bridge::SseCodec>,
) -> Result<()> {
    Err(anyhow!("anyhow error"))
}
