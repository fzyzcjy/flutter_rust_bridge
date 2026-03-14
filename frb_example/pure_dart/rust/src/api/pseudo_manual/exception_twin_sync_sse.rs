// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `exception.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::StreamSink;
use anyhow::{anyhow, Result};
use backtrace::Backtrace;
use flutter_rust_bridge::frb;

// ------------------------------ built-in errors ----------------------------------

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_return_error_twin_sync_sse() -> Result<i32> {
    Err(anyhow!("deliberate error"))
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_type_fallible_panic_twin_sync_sse() -> Result<i32> {
    panic!("deliberate panic")
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_type_infallible_panic_twin_sync_sse() -> i32 {
    panic!("deliberate panic")
}

// ------------------------------ custom error + return ok/panic ----------------------------------

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn custom_enum_error_return_ok_twin_sync_sse(
    arg: u32,
) -> Result<u32, CustomEnumErrorTwinSyncSse> {
    Ok(arg)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn custom_enum_error_panic_twin_sync_sse() -> Result<(), CustomEnumErrorTwinSyncSse> {
    panic!("deliberate panic");
}

// ------------------------------ custom struct error ----------------------------------

pub enum CustomEnumErrorTwinSyncSse {
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
#[flutter_rust_bridge::frb(sync)]
pub fn custom_enum_error_return_error_twin_sync_sse() -> Result<u32, CustomEnumErrorTwinSyncSse> {
    Err(CustomEnumErrorTwinSyncSse::One {
        message: "deliberate error".into(),
        backtrace: Backtrace::new(),
    })
}

// ------------------------------ custom nested errors ----------------------------------

pub enum CustomNestedErrorOuterTwinSyncSse {
    One(String),
    Two(CustomNestedErrorInnerTwinSyncSse),
}

pub enum CustomNestedErrorInnerTwinSyncSse {
    Three(String),
    Four(u32),
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn custom_nested_error_return_error_twin_sync_sse(
    arg: CustomNestedErrorOuterTwinSyncSse,
) -> Result<(), CustomNestedErrorOuterTwinSyncSse> {
    Err(arg)
}

// ------------------------------ custom struct errors ----------------------------------

pub struct CustomStructErrorTwinSyncSse {
    pub a: String,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn custom_struct_error_return_error_twin_sync_sse(
    arg: CustomStructErrorTwinSyncSse,
) -> Result<(), CustomStructErrorTwinSyncSse> {
    Err(arg)
}

// ------------------------------ example-based ----------------------------------

pub enum CustomErrorTwinSyncSse {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_err_custom_error_twin_sync_sse() -> Result<u32, CustomErrorTwinSyncSse> {
    Err(CustomErrorTwinSyncSse::Error0 {
        e: "".into(),
        backtrace: Backtrace::new(),
    })
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_ok_custom_error_twin_sync_sse() -> Result<u32, CustomErrorTwinSyncSse> {
    Ok(3)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_error_variant_twin_sync_sse(variant: u32) -> Result<u32, CustomErrorTwinSyncSse> {
    match variant {
        0 => Err(CustomErrorTwinSyncSse::Error0 {
            e: "variant0".to_string(),
            backtrace: Backtrace::new(),
        }),
        1 => Err(CustomErrorTwinSyncSse::Error1 {
            e: 1,
            backtrace: Backtrace::new(),
        }),
        _ => panic!("unsupported variant"),
    }
}

pub struct SomeStructTwinSyncSse {
    pub value: u32,
}

impl SomeStructTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync_sse(value: u32) -> SomeStructTwinSyncSse {
        SomeStructTwinSyncSse { value }
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_return_err_custom_error_twin_sync_sse() -> Result<u32, CustomErrorTwinSyncSse> {
        Err(CustomErrorTwinSyncSse::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_return_ok_custom_error_twin_sync_sse() -> Result<u32, CustomErrorTwinSyncSse> {
        Ok(3)
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn non_static_return_err_custom_error_twin_sync_sse(
        &self,
    ) -> Result<u32, CustomErrorTwinSyncSse> {
        Err(CustomErrorTwinSyncSse::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn non_static_return_ok_custom_error_twin_sync_sse(
        &self,
    ) -> Result<u32, CustomErrorTwinSyncSse> {
        Ok(self.value)
    }
}

pub enum CustomNestedError1TwinSyncSse {
    CustomNested1(String),
    ErrorNested(CustomNestedError2TwinSyncSse),
}

pub enum CustomNestedError2TwinSyncSse {
    CustomNested2(String),
    CustomNested2Number(u32),
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_nested_error_1_twin_sync_sse() -> Result<(), CustomNestedError1TwinSyncSse> {
    Err(CustomNestedError1TwinSyncSse::ErrorNested(
        CustomNestedError2TwinSyncSse::CustomNested2Number(3),
    ))
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_nested_error_1_variant1_twin_sync_sse(
) -> Result<(), CustomNestedError1TwinSyncSse> {
    Err(CustomNestedError1TwinSyncSse::CustomNested1(
        "custom".to_string(),
    ))
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_nested_error_2_twin_sync_sse() -> Result<(), CustomNestedError2TwinSyncSse> {
    Err(CustomNestedError2TwinSyncSse::CustomNested2(
        "custom".to_string(),
    ))
}
pub struct CustomStructErrorAnotherTwinSyncSse {
    pub message: String,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_struct_error_twin_sync_sse() -> Result<(), CustomStructErrorAnotherTwinSyncSse>
{
    Err(CustomStructErrorAnotherTwinSyncSse {
        message: "error message".to_string(),
    })
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn return_custom_struct_ok_twin_sync_sse() -> Result<u32, CustomStructErrorAnotherTwinSyncSse> {
    Ok(3)
}

pub struct CustomStructTwinSyncSse {
    pub message: String,
}

impl CustomStructTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync_sse(message: String) -> CustomStructTwinSyncSse {
        CustomStructTwinSyncSse { message }
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_return_custom_struct_error_twin_sync_sse(
    ) -> Result<(), CustomStructErrorAnotherTwinSyncSse> {
        Err(CustomStructErrorAnotherTwinSyncSse {
            message: "error message".to_string(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_return_custom_struct_ok_twin_sync_sse(
    ) -> Result<u32, CustomStructErrorAnotherTwinSyncSse> {
        Ok(3)
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn nonstatic_return_custom_struct_error_twin_sync_sse(
        &self,
    ) -> Result<(), CustomStructErrorAnotherTwinSyncSse> {
        Err(CustomStructErrorAnotherTwinSyncSse {
            message: self.message.clone(),
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn nonstatic_return_custom_struct_ok_twin_sync_sse(
        &self,
    ) -> Result<u32, CustomStructErrorAnotherTwinSyncSse> {
        Ok(3)
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn throw_anyhow_twin_sync_sse() -> Result<(), anyhow::Error> {
    Err(anyhow!("anyhow error"))
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn panic_with_custom_result_twin_sync_sse() -> Result<(), CustomErrorTwinSyncSse> {
    panic!("just a panic");
}

#[frb(stream_dart_await)]
#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn stream_sink_throw_anyhow_twin_sync_sse(
    _sink: StreamSink<String, flutter_rust_bridge::SseCodec>,
) -> Result<()> {
    Err(anyhow!("anyhow error"))
}
