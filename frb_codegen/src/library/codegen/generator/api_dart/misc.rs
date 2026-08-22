use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::utils::crate_name::CrateName;
use crate::utils::namespace::Namespace;
use itertools::concat;
use std::path::{Path, PathBuf};

pub(crate) fn compute_path_from_namespace(
    dart_decl_base_output_path: &Path,
    namespace: &Namespace,
) -> PathBuf {
    let raw_path = namespace.path();
    let chunks = if namespace == &CrateName::self_crate().namespace() {
        // workaround - for `lib.rs`, we cannot just output `/` which is invalid
        vec!["lib"]
    } else {
        match raw_path[0] {
            CrateName::SELF_CRATE => raw_path[1..].to_owned(),
            _ => concat([vec![THIRD_PARTY_DIR_NAME], raw_path.clone()]),
        }
    };

    let ans_without_extension =
        (chunks.iter()).fold(dart_decl_base_output_path.to_owned(), |a, b| a.join(b));
    ans_without_extension.with_extension("dart")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::namespace::Namespace;

    #[test]
    /// Uses lib.dart for the self-crate root namespace.
    fn maps_the_self_crate_root_to_lib_dart() {
        assert_eq!(
            compute_path_from_namespace(
                Path::new("generated"),
                &Namespace::new_raw("crate".into())
            ),
            PathBuf::from("generated/lib.dart")
        );
    }

    #[test]
    /// Removes the synthetic crate segment but retains nested self-crate modules.
    fn maps_self_crate_modules_below_the_output_root() {
        assert_eq!(
            compute_path_from_namespace(
                Path::new("generated"),
                &Namespace::new_raw("crate::api::models".into()),
            ),
            PathBuf::from("generated/api/models.dart")
        );
    }

    #[test]
    /// Places external-crate namespaces underneath third_party.
    fn maps_external_crates_under_the_third_party_directory() {
        assert_eq!(
            compute_path_from_namespace(
                Path::new("generated"),
                &Namespace::new_raw("dependency::api".into()),
            ),
            PathBuf::from("generated/third_party/dependency/api.dart")
        );
    }
}
