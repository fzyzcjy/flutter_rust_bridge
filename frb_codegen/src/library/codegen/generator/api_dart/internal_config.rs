use crate::codegen::generator::misc::target::TargetOrCommonMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorApiDartInternalConfig {
    pub dart_enums_style: bool,
    pub dart3: bool,
    pub dart_decl_base_output_path: PathBuf,
    pub dart_impl_output_path: TargetOrCommonMap<PathBuf>,
    pub dart_entrypoint_class_name: String,
    pub dart_preamble: String,
    pub dart_type_rename: HashMap<String, String>,
}
