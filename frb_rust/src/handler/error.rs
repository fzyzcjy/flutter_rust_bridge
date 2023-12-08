use crate::generalized_isolate::IntoDart;
use crate::misc::box_into_dart::BoxIntoDart;
use crate::platform_types::DartAbi;
use std::any::Any;

/// Errors that occur from normal code execution.
pub enum Error {
    /// Non-panic errors
    CustomError,
    /// Exceptional errors from panicking.
    Panic(Box<dyn Any + Send>),
}

impl Error {
    /// The message of the error.
    pub fn message(&self) -> String {
        match self {
            Error::CustomError => "Box<dyn BoxIntoDart>".to_string(),
            Error::Panic(panic_err) => error_to_string(panic_err),
        }
    }
}

fn error_to_string(panic_err: &Box<dyn Any + Send>) -> String {
    match panic_err.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match panic_err.downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<dyn Any>",
        },
    }
    .to_string()
}
