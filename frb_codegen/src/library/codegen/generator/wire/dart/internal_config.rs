use serde::{Deserialize, Serialize};

// TODO unify with `GeneratorDartInternalConfig`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireDartInternalConfig {
    pub use_bridge_in_method: bool,
    pub wasm_enabled: bool,
}
