use serde::{Deserialize, Serialize};
use crate::utils::namespace::Namespace;

/// e.g. `web-audio-api` (note the `-` instead of `_`)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub(crate) struct CrateName(String);

impl CrateName {
    pub fn new(raw: String) -> Self {
        Self(raw)
    }

    pub(crate) fn namespace(&self) -> Namespace {
        Namespace::new_raw(self.0)
    }

    pub(crate) fn raw(&self) -> &str {
        &self.0
    }
}
