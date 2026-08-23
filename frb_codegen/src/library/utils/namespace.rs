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
            "joined_path={joined_path:?} seems weird"
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
        let index = s.rfind(Self::SEP).ok_or_else(|| {
            <D::Error as serde::de::Error>::custom("namespaced name must contain `/`")
        })?;
        Ok(Self::new(
            Namespace::new_raw(s[..index].to_owned()),
            s[index + Self::SEP.len()..].to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::sse::lang::dart::DartLang;
    use crate::codegen::generator::codec::sse::lang::rust::RustLang;
    use std::path::Path;

    #[test]
    /// Joins namespace segments and exposes their display and path forms.
    fn constructs_and_displays_a_namespace() {
        // Namespace::new(vec!["crate".into(), "hello".into(), "world".into()]).to_string(),
        let namespace = Namespace::new(vec!["crate".into(), "hello".into(), "world".into()]);

        assert_eq!(namespace.to_string(), "crate::hello::world");
        assert_eq!(namespace.path(), ["crate", "hello", "world"]);
        assert_eq!(namespace.safe_ident(), "crate__hello__world");
    }

    #[test]
    /// Keeps empty namespaces empty instead of creating an empty path segment.
    fn preserves_an_empty_namespace() {
        let namespace = Namespace::new_raw(String::new());

        assert!(namespace.path().is_empty());
        assert_eq!(namespace.safe_ident(), "");
        assert_eq!(namespace.to_string(), "");
    }

    #[test]
    /// Prefixes an unqualified namespace with the synthetic crate namespace.
    fn prefixes_a_namespace_with_the_self_crate() {
        assert_eq!(
            Namespace::new_self_crate("module::child".to_owned()),
            Namespace::new_raw("crate::module::child".to_owned())
        );
    }

    #[test]
    /// Derives a self-crate namespace from a Rust source path.
    fn derives_a_namespace_from_a_rust_crate_path() -> anyhow::Result<()> {
        let namespace = Namespace::new_from_rust_crate_path(
            Path::new("/project/src/module/child.rs"),
            Path::new("/project"),
        )?;

        assert_eq!(
            namespace,
            Namespace::new_raw("crate::module::child".to_owned())
        );

        Ok(())
    }

    #[test]
    /// Returns the first namespace segment as its crate name.
    fn derives_the_crate_name_from_the_first_path_segment() {
        assert_eq!(
            Namespace::new_raw("dependency::module".to_owned())
                .crate_name()
                .raw(),
            "dependency"
        );
    }

    #[test]
    /// Appends a namespace segment using the Rust namespace separator.
    fn joins_a_namespace_segment() {
        assert_eq!(
            Namespace::new_raw("crate::module".to_owned()).join("child"),
            Namespace::new_raw("crate::module::child".to_owned())
        );
    }

    #[test]
    /// Treats prefixes as complete namespace segments rather than string prefixes.
    fn checks_namespace_prefixes_by_path_segment() {
        let prefix = Namespace::new_raw("crate::module".to_owned());

        assert!(prefix.is_prefix_of(&Namespace::new_raw("crate::module::child".to_owned())));
        assert!(Namespace::new_raw(String::new()).is_prefix_of(&prefix));
        assert!(!prefix.is_prefix_of(&Namespace::new_raw("crate::module_suffix".to_owned())));
    }

    #[test]
    /// Removes a namespace prefix while retaining the remaining path segments.
    fn strips_a_namespace_prefix() {
        assert_eq!(
            Namespace::new_raw("crate::module::child".to_owned())
                .strip_prefix(&Namespace::new_raw("crate::module".to_owned())),
            Namespace::new_raw("child".to_owned())
        );
    }

    #[test]
    /// Serializes namespaces transparently as their joined paths.
    fn serializes_a_namespace_transparently() -> anyhow::Result<()> {
        let original = Namespace::new_raw("crate::module".to_owned());
        let serialized = serde_json::to_string(&original)?;
        let restored: Namespace = serde_json::from_str(&serialized)?;

        assert_eq!(serialized, r#""crate::module""#);
        assert_eq!(restored, original);

        Ok(())
    }

    #[test]
    /// Formats namespaced names for Rust and Dart code generation contexts.
    fn formats_namespaced_names_for_each_target_language() {
        let name = NamespacedName::new(Namespace::new_raw("crate::module".into()), "Thing".into());

        assert_eq!(name.rust_style(), "crate::module::Thing");
        assert_eq!(name.style(&Lang::DartLang(DartLang)), "Thing");
        assert_eq!(
            name.style(&Lang::RustLang(RustLang)),
            "crate::module::Thing"
        );
    }

    #[test]
    /// Serializes namespaced names and splits them at their final separator.
    fn serializes_and_deserializes_namespaced_names() -> anyhow::Result<()> {
        let original = NamespacedName::new(Namespace::new_raw("a::b".into()), "c".into());
        let serialized = serde_json::to_string(&original)?;
        let recovered: NamespacedName = serde_json::from_str(&serialized)?;

        assert_eq!(serialized, r#""a::b/c""#);
        assert_eq!(original, recovered);

        Ok(())
    }

    #[test]
    /// Rejects serialized namespaced names without a namespace separator.
    fn rejects_a_name_without_a_separator_during_deserialization() {
        assert!(serde_json::from_str::<NamespacedName>(r#""name""#).is_err());
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
