use crate::ir::*;

#[derive(Debug, Clone)]
pub struct ApiFunc {
    pub name: String,
    pub inputs: Vec<ApiField>,
    pub output: ApiType,
    pub fallible: bool,
    pub mode: ApiFuncMode,
    pub comments: Vec<Comment>,
}

impl ApiFunc {
    pub fn wire_func_name(&self) -> String {
        format!("wire_{}", self.name)
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ApiFuncMode {
    Normal,
    Sync,
    Stream,
}

impl ApiFuncMode {
    pub fn dart_return_type(&self, inner: &str) -> String {
        match self {
            Self::Normal => format!("Future<{}>", inner),
            Self::Sync => inner.to_string(),
            Self::Stream => format!("Stream<{}>", inner),
        }
    }

    pub fn ffi_call_mode(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Sync => "Sync",
            Self::Stream => "Stream",
        }
    }

    pub fn has_port_argument(&self) -> bool {
        self != &Self::Sync
    }
}
