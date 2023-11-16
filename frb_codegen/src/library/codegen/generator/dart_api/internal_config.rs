use crate::codegen::config::internal_config::Namespace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorDartApiInternalConfig {
    pub dart_api_class_name: String,
    // TODO originally it is `fn get_dart_api_bridge_name()`
    pub dart_api_instance_name: String,
    pub use_bridge_in_method: bool,
}
