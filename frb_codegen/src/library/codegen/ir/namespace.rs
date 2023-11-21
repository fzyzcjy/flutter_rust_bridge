use serde::{Deserialize, Serialize};

/// The Rust files/modules/namespaces.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Namespace {
    pub(crate) path: Vec<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct NamespacedName {
    pub(crate) namespace: Namespace,
    pub(crate) name: String,
}
