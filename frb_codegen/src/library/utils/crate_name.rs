use crate::utils::namespace::Namespace;
use serde::{Deserialize, Serialize};

/// e.g. `web-audio-api` (note the `-` instead of `_`)
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(transparent)]
pub(crate) struct CrateName(String);

impl CrateName {
    pub(crate) const SELF_CRATE: &'static str = "crate";

    pub fn self_crate() -> CrateName {
        CrateName::new(Self::SELF_CRATE.to_owned())
    }

    pub fn is_self_crate(&self) -> bool {
        self.0 == Self::SELF_CRATE
    }

    pub const fn new(raw: String) -> Self {
        Self(raw)
    }

    pub(crate) fn namespace(&self) -> Namespace {
        Namespace::new_raw(self.0.replace('-', "_"))
    }

    pub(crate) fn raw(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Identifies the synthetic crate namespace exactly.
    fn identifies_only_the_self_crate() {
        assert!(CrateName::self_crate().is_self_crate());
        assert!(!CrateName::new("crate_name".to_owned()).is_self_crate());
    }

    #[test]
    /// Preserves the raw Cargo package spelling.
    fn exposes_the_original_raw_name() {
        let crate_name = CrateName::new("web-audio-api".to_owned());

        assert_eq!(crate_name.raw(), "web-audio-api");
    }

    #[test]
    /// Converts Cargo hyphens to Rust namespace underscores.
    fn converts_hyphens_when_creating_a_namespace() {
        let crate_name = CrateName::new("web-audio-api".to_owned());

        assert_eq!(
            crate_name.namespace(),
            Namespace::new_raw("web_audio_api".to_owned())
        );
    }

    #[test]
    /// Serializes and deserializes crate names transparently.
    fn serializes_transparently() -> anyhow::Result<()> {
        let original = CrateName::new("web-audio-api".to_owned());
        let serialized = serde_json::to_string(&original)?;
        let restored: CrateName = serde_json::from_str(&serialized)?;

        assert_eq!(serialized, r#""web-audio-api""#);
        assert_eq!(restored, original);

        Ok(())
    }
}
