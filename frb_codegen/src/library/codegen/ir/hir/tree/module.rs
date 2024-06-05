use crate::utils::namespace::Namespace;
use derivative::Derivative;
use itertools::concat;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct HirTreeModule {
    pub meta: HirTreeModuleMeta,
    pub content: HirTreeModuleContent,
    #[serde(skip_serializing)] // avoid too big debug dump
    pub raw: Vec<String>,
}

#[derive(Clone, Derivative, Serialize)]
#[derivative(Debug)]
pub struct HirTreeModuleMeta {
    // TODO
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct HirTreeModuleContent {
    // TODO
}
