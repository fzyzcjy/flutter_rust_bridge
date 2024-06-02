use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use crate::utils::crate_name::CrateName;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserHirInternalConfig {
    pub rust_input_namespace_pack: RustInputNamespacePack,
    pub rust_crate_dir: PathBuf,
    pub third_party_crate_names: Vec<CrateName>,
}
