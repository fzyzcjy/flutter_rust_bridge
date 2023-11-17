use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorDartApiInternalConfig {
    // TODO multi file support
    pub dart_api_class_name: String,
    // TODO originally it is `fn get_dart_api_bridge_name()`
    pub dart_api_instance_name: String,
    pub dart_enums_style: bool,
    pub use_bridge_in_method: bool,
    pub dart3: bool,
}
