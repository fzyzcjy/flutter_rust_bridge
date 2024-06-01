use crate::utils::namespace::Namespace;

/// e.g. `web-audio-api` (note the `-` instead of `_`)
pub(crate) struct CrateName(String);

impl CrateName {
    pub(crate) fn namespace(&self) -> Namespace {
        Namespace::new_raw(self.0)
    }

    pub(crate) fn raw(&self) -> &str {
        self.0
    }
}
