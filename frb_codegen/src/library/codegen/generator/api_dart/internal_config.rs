use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorApiDartInternalConfig {
    // TODO multi file support
    pub dart_api_class_name: String,
    pub dart_api_instance_name: String,
    pub dart_enums_style: bool,
    pub use_bridge_in_method: bool,
    pub dart3: bool,
}
