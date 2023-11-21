use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The Rust files/modules/namespaces.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub(crate) struct Namespace {
    // Represent via this, instead of `Vec<String>`, to avoid extra memory overhead
    joined_path: String,
}

const NAMESPACE_PATH_SEPARATOR: &str = "::";

impl Namespace {
    pub fn new(path: Vec<String>) -> Self {
        assert!((path.iter()).all(|item| !item.contains(NAMESPACE_PATH_SEPARATOR)));
        Self {
            joined_path: path.join(NAMESPACE_PATH_SEPARATOR),
        }
    }

    pub fn new_self_crate(joined_path: String) -> Self {
        assert!(!joined_path.starts_with("crate::"));
        Self {
            joined_path: format!("crate::{joined_path}"),
        }
    }

    pub fn path(&self) -> Vec<&str> {
        self.joined_path.split(NAMESPACE_PATH_SEPARATOR).collect()
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.joined_path)
    }
}

/// A name and the namespace it is in.
///
/// Usually, a name itself (say "Apple") is vague, since it can be `mod_a::Apple`
/// or `mod_b::Apple`. Instead, a namespace + name unambiguously determines the object.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub(crate) struct NamespacedName {
    pub(crate) namespace: Namespace,
    pub(crate) name: String,
}

impl NamespacedName {
    pub fn new(namespace: Namespace, name: String) -> Self {
        Self { namespace, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_namespace_display() {
        assert_eq!(
            Namespace::new(vec!["crate".into(), "hello".into(), "world".into()]).to_string(),
            "crate::hello::world"
        );
    }
}
