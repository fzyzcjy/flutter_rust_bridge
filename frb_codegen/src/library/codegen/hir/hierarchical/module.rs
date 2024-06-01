use crate::codegen::parser::source_graph::modules::{ModuleScope, ModuleSource, Visibility};
use derivative::Derivative;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct Module {
    pub(super) info: ModuleInfo,
    pub(super) scope: ModuleScope,
}

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct ModuleInfo {
    pub(super) visibility: Visibility,
    pub(super) file_path: PathBuf,
    pub(super) module_path: Vec<String>,
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    pub(super) source: ModuleSource,
}
