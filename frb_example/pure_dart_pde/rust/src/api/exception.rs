// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::frb_generated::StreamSink;
use anyhow::{anyhow, Result};
use backtrace::Backtrace;
use flutter_rust_bridge::frb;

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

// ------------------------------ example-based ----------------------------------

pub enum CustomErrorTwinNormal {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}

pub fn return_err_custom_error_twin_normal() -> Result<u32, CustomErrorTwinNormal> {
    Err(CustomErrorTwinNormal::Error0 {
        e: "".into(),
        backtrace: Backtrace::new(),
    })
}

pub fn return_ok_custom_error_twin_normal() -> Result<u32, CustomErrorTwinNormal> {
    Ok(3)
}

pub fn return_error_variant_twin_normal(variant: u32) -> Result<u32, CustomErrorTwinNormal> {
    match variant {
        0 => Err(CustomErrorTwinNormal::Error0 {
            e: "variant0".to_string(),
            backtrace: Backtrace::new(),
        }),
        1 => Err(CustomErrorTwinNormal::Error1 {
            e: 1,
            backtrace: Backtrace::new(),
        }),
        _ => panic!("unsupported variant"),
    }
}

pub struct SomeStructTwinNormal {
    pub value: u32,
}

impl SomeStructTwinNormal {
    pub fn new_twin_normal(value: u32) -> SomeStructTwinNormal {
        SomeStructTwinNormal { value }
    }

    pub fn static_return_err_custom_error_twin_normal() -> Result<u32, CustomErrorTwinNormal> {
        Err(CustomErrorTwinNormal::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    pub fn static_return_ok_custom_error_twin_normal() -> Result<u32, CustomErrorTwinNormal> {
        Ok(3)
    }

    pub fn non_static_return_err_custom_error_twin_normal(
        &self,
    ) -> Result<u32, CustomErrorTwinNormal> {
        Err(CustomErrorTwinNormal::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    pub fn non_static_return_ok_custom_error_twin_normal(
        &self,
    ) -> Result<u32, CustomErrorTwinNormal> {
        Ok(self.value)
    }
}

pub enum CustomNestedError1TwinNormal {
    CustomNested1(String),
    ErrorNested(CustomNestedError2TwinNormal),
}

pub enum CustomNestedError2TwinNormal {
    CustomNested2(String),
    CustomNested2Number(u32),
}

pub fn return_custom_nested_error_1_twin_normal() -> Result<(), CustomNestedError1TwinNormal> {
    Err(CustomNestedError1TwinNormal::ErrorNested(
        CustomNestedError2TwinNormal::CustomNested2Number(3),
    ))
}

pub fn return_custom_nested_error_1_variant1_twin_normal(
) -> Result<(), CustomNestedError1TwinNormal> {
    Err(CustomNestedError1TwinNormal::CustomNested1(
        "custom".to_string(),
    ))
}

pub fn return_custom_nested_error_2_twin_normal() -> Result<(), CustomNestedError2TwinNormal> {
    Err(CustomNestedError2TwinNormal::CustomNested2(
        "custom".to_string(),
    ))
}
pub struct CustomStructErrorAnotherTwinNormal {
    pub message: String,
}

pub fn return_custom_struct_error_twin_normal() -> Result<(), CustomStructErrorAnotherTwinNormal> {
    Err(CustomStructErrorAnotherTwinNormal {
        message: "error message".to_string(),
    })
}

pub fn return_custom_struct_ok_twin_normal() -> Result<u32, CustomStructErrorAnotherTwinNormal> {
    Ok(3)
}

pub struct CustomStructTwinNormal {
    pub message: String,
}

impl CustomStructTwinNormal {
    pub fn new_twin_normal(message: String) -> CustomStructTwinNormal {
        CustomStructTwinNormal { message }
    }

    pub fn static_return_custom_struct_error_twin_normal(
    ) -> Result<(), CustomStructErrorAnotherTwinNormal> {
        Err(CustomStructErrorAnotherTwinNormal {
            message: "error message".to_string(),
        })
    }

    pub fn static_return_custom_struct_ok_twin_normal(
    ) -> Result<u32, CustomStructErrorAnotherTwinNormal> {
        Ok(3)
    }

    pub fn nonstatic_return_custom_struct_error_twin_normal(
        &self,
    ) -> Result<(), CustomStructErrorAnotherTwinNormal> {
        Err(CustomStructErrorAnotherTwinNormal {
            message: self.message.clone(),
        })
    }

    pub fn nonstatic_return_custom_struct_ok_twin_normal(
        &self,
    ) -> Result<u32, CustomStructErrorAnotherTwinNormal> {
        Ok(3)
    }
}

pub fn throw_anyhow_twin_normal() -> Result<(), anyhow::Error> {
    Err(anyhow!("anyhow error"))
}

pub fn panic_with_custom_result_twin_normal() -> Result<(), CustomErrorTwinNormal> {
    panic!("just a panic");
}

#[frb(stream_dart_await)]
pub fn stream_sink_throw_anyhow_twin_normal(_sink: StreamSink<String>) -> Result<()> {
    Err(anyhow!("anyhow error"))
}
