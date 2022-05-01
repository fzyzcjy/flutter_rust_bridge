#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Self { msg: error }
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Self {
            msg: error.to_string(),
        }
    }
}

impl std::error::Error for Error {}
