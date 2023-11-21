use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The Rust files/modules/namespaces.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct Namespace {
    pub(crate) path: Vec<String>,
}

impl Namespace {
    pub fn new(path: Vec<String>) -> Self {
        Self { path }
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path.join("::"))
    }
}

/// A name and the namespace it is in.
///
/// Usually, a name itself (say "Apple") is vague, since it can be `mod_a::Apple`
/// or `mod_b::Apple`. Instead, a namespace + name unambiguously determines the object.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct NamespacedName {
    pub(crate) namespace: Namespace,
    pub(crate) name: String,
}

impl NamespacedName {
    pub fn new(namespace: Vec<String>, name: String) -> Self {
        Self {
            namespace: Namespace::new(namespace),
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_namespace_display() {
        assert_eq!(
            Namespace::new(vec!["crate".into(), "hello".into(), "world".into()]),
            "crate::hello::world"
        );
    }
}
