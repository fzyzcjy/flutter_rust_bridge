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
            Error::CustomError => "CustomError".to_string(),
            Error::Panic(panic_err) => error_to_string(panic_err),
        }
    }
}

pub(crate) fn error_to_string(panic_err: &Box<dyn Any + Send>) -> String {
    match panic_err.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match panic_err.downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<dyn Any>",
        },
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use crate::handler::error::Error;

    #[test]
    fn test_error_message() {
        assert_eq!(Error::CustomError.message(), "CustomError".to_owned());
        assert_eq!(
            Error::Panic(Box::new(42)).message(),
            "Box<dyn Any>".to_owned()
        );
    }
}
