use crate::generalized_isolate::IntoDart;
use crate::handler::error::Error;
use crate::platform_types::DartAbi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rust2DartAction {
    Success = 0,
    Error = 1, // TODO rename?
    CloseStream = 2,
    Panic = 3,
}

impl From<&Error> for Rust2DartAction {
    fn from(value: &Error) -> Self {
        match value {
            Error::CustomError => Self::Error,
            Error::Panic(_) => Self::Panic,
        }
    }
}

impl IntoDart for Rust2DartAction {
    fn into_dart(self) -> DartAbi {
        (self as i32).into_dart()
    }
}
