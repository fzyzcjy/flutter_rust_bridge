use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct PreparerInternalConfig {
    pub dart_root: PathBuf,
    pub deps_check: bool,
    pub needs_ffigen: bool,
}

#[cfg(test)]
mod tests {
    use super::PreparerInternalConfig;
    use std::path::PathBuf;

    /// Round-trips the preparer declaration without invoking tool checks.
    #[test]
    fn test_preparer_internal_config_is_a_serde_declaration_carrier() {
        let config = PreparerInternalConfig {
            dart_root: PathBuf::from("dart"),
            deps_check: true,
            needs_ffigen: true,
        };

        let encoded = serde_json::to_string(&config).unwrap();
        assert_eq!(
            serde_json::from_str::<PreparerInternalConfig>(&encoded).unwrap(),
            config
        );
    }
}
