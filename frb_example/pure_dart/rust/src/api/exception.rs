use anyhow::{anyhow, Result};
use backtrace::Backtrace;
use flutter_rust_bridge::StreamSink;

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

pub enum CustomError {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}

pub fn return_err_custom_error() -> Result<u32, CustomError> {
    Err(CustomError::Error0 {
        e: "".into(),
        backtrace: Backtrace::new(),
    })
}

pub fn return_ok_custom_error() -> Result<u32, CustomError> {
    Ok(3)
}

pub fn return_error_variant(variant: u32) -> Result<u32, CustomError> {
    match variant {
        0 => Err(CustomError::Error0 {
            e: "variant0".to_string(),
            backtrace: Backtrace::new(),
        }),
        1 => Err(CustomError::Error1 {
            e: 1,
            backtrace: Backtrace::new(),
        }),
        _ => panic!("unsupported variant"),
    }
}

pub struct SomeStruct {
    pub value: u32,
}

impl SomeStruct {
    pub fn new(value: u32) -> SomeStruct {
        SomeStruct { value }
    }

    pub fn static_return_err_custom_error() -> Result<u32, CustomError> {
        Err(CustomError::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    pub fn static_return_ok_custom_error() -> Result<u32, CustomError> {
        Ok(3)
    }

    pub fn non_static_return_err_custom_error(&self) -> Result<u32, CustomError> {
        Err(CustomError::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    pub fn non_static_return_ok_custom_error(&self) -> Result<u32, CustomError> {
        Ok(self.value)
    }
}

pub enum CustomNestedError1 {
    CustomNested1(String),
    ErrorNested(CustomNestedError2),
}

pub enum CustomNestedError2 {
    CustomNested2(String),
    CustomNested2Number(u32),
}

pub fn return_custom_nested_error_1() -> Result<(), CustomNestedError1> {
    Err(CustomNestedError1::ErrorNested(
        CustomNestedError2::CustomNested2Number(3),
    ))
}

pub fn return_custom_nested_error_1_variant1() -> Result<(), CustomNestedError1> {
    Err(CustomNestedError1::CustomNested1("custom".to_string()))
}

pub fn return_custom_nested_error_2() -> Result<(), CustomNestedError2> {
    Err(CustomNestedError2::CustomNested2("custom".to_string()))
}
pub struct CustomStructError {
    pub message: String,
}

pub fn return_custom_struct_error() -> Result<(), CustomStructError> {
    Err(CustomStructError {
        message: "error message".to_string(),
    })
}

#[flutter_rust_bridge::frb(sync)]
pub fn sync_return_custom_struct_error() -> Result<(), CustomStructError> {
    Err(CustomStructError {
        message: "error message".to_string(),
    })
}

pub fn return_custom_struct_ok() -> Result<u32, CustomStructError> {
    Ok(3)
}

pub struct CustomStruct {
    pub message: String,
}

impl CustomStruct {
    pub fn new(message: String) -> CustomStruct {
        CustomStruct { message }
    }

    pub fn static_return_custom_struct_error() -> Result<(), CustomStructError> {
        Err(CustomStructError {
            message: "error message".to_string(),
        })
    }

    pub fn static_return_custom_struct_ok() -> Result<u32, CustomStructError> {
        Ok(3)
    }

    pub fn nonstatic_return_custom_struct_error(&self) -> Result<(), CustomStructError> {
        Err(CustomStructError {
            message: self.message.clone(),
        })
    }

    pub fn nonstatic_return_custom_struct_ok(&self) -> Result<u32, CustomStructError> {
        Ok(3)
    }
}

pub fn throw_anyhow() -> Result<(), anyhow::Error> {
    Err(anyhow!("anyhow error"))
}

pub fn panic_with_custom_result() -> Result<(), CustomError> {
    panic!("just a panic");
}

pub fn stream_sink_throw_anyhow(_sink: StreamSink<String>) -> Result<()> {
    Err(anyhow!("anyhow error"))
}
