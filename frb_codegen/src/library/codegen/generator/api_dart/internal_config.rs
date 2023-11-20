use crate::codegen::config::internal_config::Namespace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorApiDartInternalConfig {
    pub dart_enums_style: bool,
    pub dart3: bool,
    pub dart_decl_output_path: HashMap<Namespace, PathBuf>,
}
