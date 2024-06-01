use crate::codegen::parser::mir::internal_config::RustInputNamespacePack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserHirInternalConfig {
    pub rust_input_namespace_pack: RustInputNamespacePack,
}
