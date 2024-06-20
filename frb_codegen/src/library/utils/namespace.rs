use crate::codegen::generator::codec::sse::lang::Lang;
use crate::utils::crate_name::CrateName;
use crate::utils::rust_project_utils::compute_mod_from_rust_crate_path;
use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::ToOwned;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// The Rust files/modules/namespaces.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd, Default)]
#[serde(transparent)]
pub struct Namespace {
    // Represent via this, instead of `Vec<String>`, to avoid extra memory overhead
    pub(crate) joined_path: String,
}

impl Namespace {
    pub(crate) const SEP: &'static str = "::";

    pub fn new(path: Vec<String>) -> Self {
        assert!((path.iter()).all(|item| !item.contains(Self::SEP)));
        Self::new_raw(path.join(Self::SEP))
    }

    pub fn new_raw(joined_path: String) -> Self {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        assert!(
            !joined_path.contains('\\'),
            "joined_path={:?} seems weird",
            joined_path
        );
        // frb-coverage:ignore-end
        Self { joined_path }
    }

    pub fn new_self_crate(joined_path: String) -> Self {
        let sep = Self::SEP;
        let self_crate = CrateName::SELF_CRATE;

        assert!(!joined_path.starts_with(&format!("{self_crate}{sep}")));
        Self::new_raw(format!("{self_crate}{sep}{joined_path}"))
    }

    pub(crate) fn new_from_rust_crate_path(
        code_path: &Path,
        rust_crate_path: &Path,
    ) -> anyhow::Result<Self> {
        let p = compute_mod_from_rust_crate_path(code_path, rust_crate_path)?;
        Ok(Self::new_self_crate(p))
    }

    pub fn crate_name(&self) -> CrateName {
        CrateName::new(self.path()[0].to_owned())
    }

    pub fn path(&self) -> Vec<&str> {
        if self.joined_path.is_empty() {
            return vec![];
        }
        self.joined_path.split(Self::SEP).collect()
    }

    // pub fn path_exclude_self_crate(&self) -> Vec<&str> {
    //     let mut path = self.path();
    //     if path.first() == Some(&CrateName::SELF_CRATE) {
    //         path.remove(0);
    //     }
    //     path
    // }

    // pub fn to_pseudo_io_path(&self, extension: &str) -> PathBuf {
    //     PathBuf::from(&format!("/{}.{extension}", self.path().join("/")))
    // }

    pub fn safe_ident(&self) -> String {
        self.path().iter().join("__")
    }

    pub fn join(&self, other: &str) -> Self {
        Self::new_raw(format!("{}{}{}", self.joined_path, Self::SEP, other))
    }

    pub fn is_prefix_of(&self, other: &Namespace) -> bool {
        other.path().starts_with(&self.path())
    }

    pub fn strip_prefix(&self, prefix: &Namespace) -> Self {
        let self_path = (self.path().into_iter().map(ToString::to_string)).collect_vec();
        let prefix_path = (prefix.path().into_iter().map(ToString::to_string)).collect_vec();
        Self::new(self_path.strip_prefix(&prefix_path[..]).unwrap().to_vec())
    }

    // pub fn compute_common_prefix(namespaces: &[&Namespace]) -> Self {
    //     let paths = namespaces.iter().map(|x| x.path()).collect_vec();
    //     let prefix_len = vec_common_prefix(&paths);
    //     Self::new(
    //         paths[0][..prefix_len]
    //             .iter()
    //             .map(|x| x.to_string())
    //             .collect_vec(),
    //     )
    // }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.joined_path)
    }
}

// fn vec_common_prefix(vecs: &[Vec<&str>]) -> usize {
//     let min_len = vecs.iter().map(|x| x.len()).reduce(usize::min).unwrap();
//     for i in 0..min_len {
//         let sample_value = vecs[0][i];
//         if vecs.iter().any(|vec| vec[i] != sample_value) {
//             return i;
//         }
//     }
//     min_len
// }

/// A name and the namespace it is in.
///
/// Usually, a name itself (say "Apple") is vague, since it can be `mod_a::Apple`
/// or `mod_b::Apple`. Instead, a namespace + name unambiguously determines the object.
// Note: Do NOT implement `display`, otherwise it is easy to misuse it.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct NamespacedName {
    pub namespace: Namespace,
    pub name: String,
}

impl NamespacedName {
    const SEP: &'static str = "/";

    pub fn new(namespace: Namespace, name: String) -> Self {
        Self { namespace, name }
    }

    pub fn rust_style(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }

    pub fn style(&self, lang: &Lang) -> String {
        match lang {
            Lang::DartLang(_) => self.name.clone(),
            Lang::RustLang(_) => self.rust_style(),
        }
    }
}

impl Serialize for NamespacedName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{}{}{}", self.namespace, Self::SEP, self.name).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NamespacedName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let index = s.rfind(Self::SEP).unwrap();
        Ok(Self::new(
            Namespace::new_raw(s[..index].to_owned()),
            s[index + Self::SEP.len()..].to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_namespace_display() {
        assert_eq!(
            // Namespace::new(vec!["crate".into(), "hello".into(), "world".into()]).to_string(),
            Namespace::new_raw("crate::hello::world".to_owned()).to_string(),
            "crate::hello::world"
        );
    }

    #[test]
    pub fn test_namespaced_name_serialization() -> anyhow::Result<()> {
        let original = NamespacedName::new(Namespace::new_raw("a::b".into()), "c".into());
        let serialized = serde_json::to_string(&original)?;
        let recovered: NamespacedName = serde_json::from_str(&serialized)?;

        assert_eq!(serialized, r#""a::b/c""#);
        assert_eq!(original, recovered);

        Ok(())
    }

    // #[test]
    // pub fn test_to_pseudo_io_path() -> anyhow::Result<()> {
    //     assert_eq!(
    //         Namespace::new_raw("apple::orange".into()).to_pseudo_io_path("dart"),
    //         PathBuf::from("/apple/orange.dart")
    //     );
    //     Ok(())
    // }
}
